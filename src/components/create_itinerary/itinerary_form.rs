
use leptos::*;

#[component]
pub fn ItineraryForm(cx: Scope) -> impl IntoView {
    let (title, set_title) = create_signal(cx, String::new());
    let (start_date, set_start_date) = create_signal(cx, String::new());
    let (end_date, set_end_date) = create_signal(cx, String::new());
    let (description, set_description) = create_signal(cx, String::new());

    view! { cx,
        <form class="space-y-4">
            <div>
                <label for="title" class="block text-sm font-medium text-gray-700">"Title"</label>
                <input
                    id="title"
                    type="text"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    on:input=move |ev| set_title(event_target_value(&ev))
                    prop:value=title
                />
            </div>
            <div>
                <label for="start_date" class="block text-sm font-medium text-gray-700">"Start Date"</label>
                <input
                    id="start_date"
                    type="date"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    on:input=move |ev| set_start_date(event_target_value(&ev))
                    prop:value=start_date
                />
            </div>
            <div>
                <label for="end_date" class="block text-sm font-medium text-gray-700">"End Date (optional)"</label>
                <input
                    id="end_date"
                    type="date"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    on:input=move |ev| set_end_date(event_target_value(&ev))
                    prop:value=end_date
                />
            </div>
            <div>
                <label for="description" class="block text-sm font-medium text-gray-700">"Description"</label>
                <textarea
                    id="description"
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                    on:input=move |ev| set_description(event_target_value(&ev))
                    prop:value=description
                ></textarea>
            </div>
            <button type="submit" class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                "Create Itinerary"
            </button>
        </form>
    }
}
