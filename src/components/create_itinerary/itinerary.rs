
use leptos::*;

#[component]
pub fn Itinerary(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="max-w-2xl mx-auto p-4">
            <h1 class="text-3xl font-bold mb-4">"Travel Itinerary"</h1>
            <ItineraryForm />
            <ItineraryDetails>
                <ItineraryActivities />
            </ItineraryDetails>
        </div>
    }
}
