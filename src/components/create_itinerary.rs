mod itinerary;
mod itinerary_form;
mod itinerary_activities;

mod itinerary_details {
    use leptos::*;

    #[component]
    pub fn ItineraryDetails(cx: Scope, children: Children) -> impl IntoView {
        view! { cx,
            <div class="mt-8">
                <h2 class="text-2xl font-semibold mb-4">"Itinerary Details"</h2>
                <div class="bg-white shadow overflow-hidden sm:rounded-lg">
                    <div class="px-4 py-5 sm:px-6">
                        <h3 class="text-lg leading-6 font-medium text-gray-900">"Trip to Paris"</h3>
                        <p class="mt-1 max-w-2xl text-sm text-gray-500">"June 15, 2023 - June 22, 2023"</p>
                    </div>
                    <div class="border-t border-gray-200">
                        <dl>
                            <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                <dt class="text-sm font-medium text-gray-500">"Description"</dt>
                                <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">"A week-long adventure in the City of Light"</dd>
                            </div>
                        </dl>
                    </div>
                    <div class="px-4 py-5 sm:px-6">
                        {children(cx)}
                    </div>
                </div>
            </div>
        }
    }
}
