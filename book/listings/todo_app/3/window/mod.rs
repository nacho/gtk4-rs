mod imp;

use crate::task_object::{TaskData, TaskObject};
use crate::task_row::TaskRow;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{
    Application, CustomFilter, FilterListModel, NoSelection, SignalListItemFactory,
};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create `Window`.")
    }

    fn model(&self) -> &gio::ListStore {
        // Get state
        self.imp().model.get().expect("Could not get model")
    }

    fn filter(&self) -> Option<CustomFilter> {
        // Get state

        // Get filter state from settings
        let filter_state: String = self.imp().settings.get("filter");

        // Create custom filters
        let filter_open = CustomFilter::new(|obj| {
            // Get `TaskObject` from `glib::Object`
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow completed tasks
            !task_object.is_completed()
        });
        let filter_done = CustomFilter::new(|obj| {
            // Get `TaskObject` from `glib::Object`
            let task_object = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow done tasks
            task_object.is_completed()
        });

        // Return the correct filter
        match filter_state.as_str() {
            "All" => None,
            "Open" => Some(filter_open),
            "Done" => Some(filter_done),
            _ => unreachable!(),
        }
    }

    fn setup_model(&self) {
        // Create new model
        let model = gio::ListStore::new(TaskObject::static_type());

        // Get state and set model
        self.imp().model.set(model).expect("Could not set model");

        // Wrap model with filter and selection and pass it to the list view
        let filter_model =
            FilterListModel::new(Some(self.model()), self.filter().as_ref());
        let selection_model = NoSelection::new(Some(&filter_model));
        self.imp().list_view.set_model(Some(&selection_model));

        // Filter model whenever the value of the key "filter" changes
        self.imp().settings.connect_changed(
            Some("filter"),
            clone!(@weak self as window, @weak filter_model => move |_, _| {
                filter_model.set_filter(window.filter().as_ref());
            }),
        );
    }

    fn restore_data(&self) {
        // Deserialize data from file to vector
        let tasks: Vec<TaskData> = self.imp().settings.get("tasks");

        // Convert `Vec<TaskData>` to `Vec<TaskObject>`
        let task_objects: Vec<TaskObject> = tasks
            .into_iter()
            .map(|todo_data| TaskObject::new(todo_data.completed, todo_data.content))
            .collect();

        // Insert restored objects into model
        self.model().splice(0, 0, &task_objects);
    }

    fn setup_callbacks(&self) {
        // Setup callback for activation of the entry
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_task();
            }));

        // Setup callback for clicking (and the releasing) the icon of the entry
        self.imp().entry.connect_icon_release(
            clone!(@weak self as window => move |_,_| {
                window.new_task();
            }),
        );
    }

    fn new_task(&self) {
        // Get content from entry and clear it
        let buffer = self.imp().entry.buffer();
        let content = buffer.text();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");

        // Add new task to model
        let task = TaskObject::new(false, content);
        self.model().append(&task);
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `TaskRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `TaskRow`
            let task_row = TaskRow::new();
            list_item.set_child(Some(&task_row));
        });

        // Tell factory how to bind `TaskRow` to a `TaskObject`
        factory.connect_bind(move |_, list_item| {
            // Get `TaskObject` from `ListItem`
            let task_object = list_item
                .item()
                .expect("The item has to exist.")
                .downcast::<TaskObject>()
                .expect("The item has to be an `TaskObject`.");

            // Get `TaskRow` from `ListItem`
            let task_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.bind(&task_object);
        });

        // Tell factory how to unbind `TaskRow` from `TaskObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `TaskRow` from `ListItem`
            let task_row = list_item
                .child()
                .expect("The child has to exist.")
                .downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.unbind();
        });

        // Set the factory of the list view
        self.imp().list_view.set_factory(Some(&factory));
    }

    fn setup_actions(&self) {
        // Create action from key "filter" and add to action group "win"
        let action_filter = self.imp().settings.create_action("filter");
        self.add_action(&action_filter);

        // Get model
        let model = self.model();

        // Create action to remove done tasks and add to action group "win"
        let action_remove_done_tasks =
            gio::SimpleAction::new("remove-done-tasks", None);
        action_remove_done_tasks.connect_activate(clone!(@weak model => move |_, _| {
            let mut position = 0;
            while let Some(item) = model.item(position) {
                // Get `TaskObject` from `glib::Object`
                let task_object = item
                    .downcast_ref::<TaskObject>()
                    .expect("The object needs to be of type `TaskObject`.");

                if task_object.is_completed() {
                    model.remove(position);
                } else {
                    position += 1;
                }
            }
        }));
        self.add_action(&action_remove_done_tasks);
    }
}
