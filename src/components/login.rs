use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="bg-gradient-to-r from-blue-600 to-blue-800 flex w-screen h-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <div class="bg-white rounded-xl shadow-2xl p-8 max-w-md w-full transform transition-all hover:scale-105 duration-300">
                    <div class="text-center mb-8">
                        <div class="inline-block p-4 bg-blue-50 rounded-full mb-4">
                            // Profile icon - you can replace this with an actual SVG or image if needed
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                            </svg>
                        </div>
                        <h1 class="text-3xl font-bold text-gray-800">{"Welcome to YewChat"}</h1>
                        <p class="text-gray-500 mt-2">{"Connect with friends in real-time"}</p>
                    </div>
                    
                    <div class="mt-6">
                        <label class="block text-sm font-medium text-gray-700 mb-2">{"Username"}</label>
                        <div class="relative">
                            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd" />
                                </svg>
                            </div>
                            <input 
                                {oninput} 
                                class="pl-10 block w-full border-gray-300 rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500 transition-all duration-200" 
                                placeholder="Enter your username" 
                            />
                        </div>
                    </div>
                    
                    <div class="mt-8">
                        <Link<Route> to={Route::Chat} classes="w-full block"> 
                            <button 
                                {onclick} 
                                disabled={username.len()<1} 
                                class="w-full flex justify-center py-3 px-4 rounded-lg bg-gradient-to-r from-blue-500 to-blue-700 text-white font-medium shadow-lg hover:from-blue-600 hover:to-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-all duration-200 disabled:opacity-50"
                            >
                                <span>{"Start Chatting!"}</span>
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 ml-2" viewBox="0 0 20 20" fill="currentColor">
                                    <path fill-rule="evenodd" d="M10.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L12.586 11H5a1 1 0 110-2h7.586l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd" />
                                </svg>
                            </button>
                        </Link<Route>>
                    </div>
                    
                    <p class="text-center text-sm text-gray-500 mt-6">
                        {"Creative communication starts with a unique identity"}
                    </p>
                </div>
                
                <div class="mt-8 text-sm text-white text-center">
                    {"© 2023 YewChat • Powered by Rust & WebAssembly"}
                </div>
            </div>
        </div>
    }
}