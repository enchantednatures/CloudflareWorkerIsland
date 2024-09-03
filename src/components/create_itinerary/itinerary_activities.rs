use leptos::*;

#[component]
pub fn ItineraryActivities(cx: Scope) -> impl IntoView {
    let (activities, set_activities) = create_signal(cx, vec![
        "Visit the Eiffel Tower".to_string(),
        "Explore the Louvre Museum".to_string(),
        "Stroll along the Seine River".to_string(),
    ]);

    let (new_activity, set_new_activity) = create_signal(cx, String::new());

    let add_activity = move |_| {
        if !new_activity.get().is_empty() {
            set_activities.update(|acts| acts.push(new_activity.get()));
            set_new_activity.set(String::new());
        }
    };

    view! { cx,
        <div>
            <h3 class="text-xl font-semibold mb-2">"Activities"</h3>
            <ul class="list-disc list-inside mb-4">
                {move || activities.get().into_iter().map(|activity| view! { cx, <li>{activity}</li> }).collect::<Vec<_>>()}
            </ul>
            <div class="flex">
                <input
                    type="text"
                    class="flex-grow mr-2 rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    placeholder="Add new activity"
                    on:input=move |ev| set_new_activity(event_target_value(&ev))
                    prop:value=new_activity
                />
                <button
                    class="px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    on:click=add_activity
                >
                    "Add"
                </button>
            </div>
        </div>
    }
}
