use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Some(Msg::SubmitMessage)
            } else {
                None
            }
        });

        html! {
            <div class="flex w-full h-screen bg-gray-100 overflow-hidden">
                <div class="hidden md:flex flex-col w-80 bg-white shadow-lg">
                    <div class="p-4 border-b border-gray-200 bg-blue-700 text-white">
                        <h2 class="text-xl font-bold flex items-center">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                            </svg>
                            {"Online Users"}
                        </h2>
                    </div>
                    <div class="overflow-y-auto flex-1">
                        {
                            if self.users.is_empty() {
                                html! {
                                    <div class="flex flex-col items-center justify-center h-32 text-gray-500">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                        </svg>
                                        <p>{"No users online"}</p>
                                    </div>
                                }
                            } else {
                                self.users.clone().iter().map(|u| {
                                    html!{
                                        <div class="flex items-center p-4 border-b border-gray-100 hover:bg-gray-50 transition-colors duration-150 cursor-pointer">
                                            <div class="relative">
                                                <img class="w-12 h-12 rounded-full object-cover border-2 border-blue-400" src={u.avatar.clone()} alt="avatar"/>
                                                <div class="absolute bottom-0 right-0 w-3 h-3 bg-blue-400 rounded-full border-2 border-white"></div>
                                            </div>
                                            <div class="ml-4">
                                                <h3 class="font-semibold">{u.name.clone()}</h3>
                                                <p class="text-xs text-gray-500">{"Online"}</p>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                </div>

                <div class="flex flex-col flex-1 bg-white overflow-hidden">
                    <div class="flex items-center px-6 py-3 border-b border-gray-200 shadow-sm">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-blue-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z" />
                        </svg>
                        <h1 class="text-xl font-bold text-gray-700">{"YewChat"}</h1>
                        <div class="ml-auto text-sm text-gray-500 flex items-center">
                            <span class="mr-1">{"Active users:"}</span>
                            <span class="bg-blue-100 text-blue-800 px-2 py-0.5 rounded-full font-medium">
                                {self.users.len().to_string()}
                            </span>
                        </div>
                    </div>

                    <div class="flex-1 p-6 overflow-y-auto bg-gray-50">
                        {
                            if self.messages.is_empty() {
                                html! {
                                    <div class="flex flex-col items-center justify-center h-full text-gray-500">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mb-4 text-gray-300" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                                        </svg>
                                        <p class="text-lg font-medium">{"No messages yet"}</p>
                                        <p class="mt-1">{"Start chatting by typing a message below"}</p>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="space-y-6">
                                        {
                                            self.messages.iter().map(|m| {
                                                let user_found = self.users.iter().find(|u| u.name == m.from);
                                                let user = user_found.cloned().unwrap_or_else(|| UserProfile {
                                                            name: m.from.clone(),
                                                            avatar: format!("https://avatars.dicebear.com/api/identicon/{}.svg", m.from)
                                                        });
                                                html!{
                                                    <div class="flex items-start">
                                                        <img class="w-10 h-10 rounded-full mr-3 shadow" src={user.avatar.clone()} alt="avatar"/>
                                                        <div class="flex flex-col max-w-3xl">
                                                            <div class="flex items-center">
                                                                <span class="font-semibold text-gray-800">{user.name.clone()}</span>
                                                                <span class="text-xs text-gray-400 ml-2">{"just now"}</span>
                                                            </div>
                                                            <div class={format!("mt-1 p-3 bg-white rounded-lg shadow-sm border-l-4 {}", if m.from == "You" {"border-blue-500"} else {"border-blue-300"} )}>
                                                                {
                                                                    if m.message.ends_with(".gif") {
                                                                        html! {
                                                                            <div class="mt-1 rounded-md overflow-hidden">
                                                                                <img class="max-w-full rounded" src={m.message.clone()} alt="GIF"/>
                                                                            </div>
                                                                        }
                                                                    } else {
                                                                        html! {
                                                                            <p class="text-gray-700">{m.message.clone()}</p>
                                                                        }
                                                                    }
                                                                }
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                }
                            }
                        }
                    </div>

                    <div class="p-4 border-t border-gray-200 bg-white">
                        <div class="flex rounded-lg border border-gray-300 overflow-hidden shadow-sm focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-blue-500">
                            <input 
                                ref={self.chat_input.clone()} 
                                type="text" 
                                placeholder="Type your message..." 
                                class="flex-1 px-4 py-3 focus:outline-none" 
                                onkeypress={onkeypress}
                            />
                            <div class="flex items-center px-2 bg-gray-50 border-l border-gray-300">
                                <button 
                                    class="p-2 rounded-full text-gray-400 hover:text-gray-600 focus:outline-none"
                                    title="Insert emoji"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                </button>
                                <button 
                                    onclick={submit} 
                                    class="ml-2 px-4 py-2 bg-blue-600 text-white font-medium rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors duration-150 flex items-center"
                                >
                                    <span class="mr-1">{"Send"}</span>
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                        <div class="flex items-center justify-between text-xs text-gray-500 mt-2 px-2">
                            <div>{"Type @username to mention a user"}</div>
                            <div>{"Enter to send, Shift+Enter for new line"}</div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
