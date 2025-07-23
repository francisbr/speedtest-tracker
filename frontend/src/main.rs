use crate::component::SideBar;
use crate::page::home::HomePage;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

mod component;
mod page;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <Router>
                <main>
                    <SideBar />

                    <div class="p-4 sm:ml-64">
                        <Routes fallback=FallbackView>
                            <Route path=path!("") view=HomePage />
                        </Routes>
                    </div>
                </main>
            </Router>

            <script src="https://cdn.jsdelivr.net/npm/flowbite@3.1.2/dist/flowbite.min.js"></script>
        }
    })
}

#[component]
pub fn fallback_view() -> impl IntoView {
    view! {
        <section class="bg-white dark:bg-gray-900">
            <div class="py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16">
                <h1 class="mb-4 text-4xl font-extrabold tracking-tight leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white">
                    "404 Page not found..."
                </h1>
                <p class="mb-8 text-lg font-normal text-gray-500 lg:text-xl sm:px-16 lg:px-48 dark:text-gray-400">
                    "You've reached an unknown place on the internet. We can't help you with that"
                </p>
                <div class="flex flex-col space-y-4 sm:flex-row sm:justify-center sm:space-y-0">
                    <A href="/">
                        <div class="inline-flex justify-center items-center py-3 px-5 text-base font-medium text-center text-white rounded-lg bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 dark:focus:ring-blue-900">
                            Get started
                            <svg
                                class="w-3.5 h-3.5 ms-2 rtl:rotate-180"
                                aria-hidden="true"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 14 10"
                            >
                                <path
                                    stroke="currentColor"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M1 5h12m0 0L9 1m4 4L9 9"
                                />
                            </svg>
                        </div>
                    </A>
                </div>
            </div>
        </section>
    }
}
