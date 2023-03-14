use crate::components::{
    icons::*,
    messages::*,
};

use common::{
    types::*,
};

use leptos::*;

#[component]
pub fn Dispatchers(cx: Scope) -> impl IntoView {
    let dispatchers = use_context::<ReadSignal<NextflowDispatchers>>(cx).unwrap();
    let set_dispatchers = use_context::<WriteSignal<NextflowDispatchers>>(cx).unwrap();

    let (show, set_show) = create_signal(cx, false);
    let (new_dispatcher_url, set_new_dispatcher_url) = create_signal(cx, "".to_string());
    
    let on_click_add = move |_| {
        log!("{:#?}", show.get());
        set_show.update(|b| *b = !*b);
    };

    let on_click_save = move |_| {
        log!("save: {}", new_dispatcher_url.get());

        set_dispatchers.update(
            |dispatchers| dispatchers.add(
                NextflowDispatcher::new(
                    Uuid::new_v4(), 
                    new_dispatcher_url.get()
                )
            )
        );

        set_new_dispatcher_url.set("".to_string());
        set_show.update(|b| *b = !*b) 
    };
    
    let on_click_cancel = move |_| {
        log!("cancel");
        set_new_dispatcher_url.set("".to_string());
        set_show.update(|b| *b = !*b) 
    };

    let on_input_url = move |ev| {
        set_new_dispatcher_url.set(event_target_value(&ev));
    };

    view!{cx,
        <div class="my-1 mx-2">
            <div class="flex">
                <h3 class="grow text-xl">"Dispatchers"</h3>
                <Show 
                    when={move || show.get()}
                    fallback=|_cx| view! { cx, }
                >
                    <div class="absolute inset-0 bg-black bg-opacity-30 h-screen w-full flex justify-center items-start md:items-center pt-10 md:pt-0">
                    <div class="bg-gray-100 rounded px-4 py-4">
                    <div class="flex">
                        <h2>"Add dispatcher"</h2>
                        <div class="grow" />
                        <IconButton 
                            kind=ButtonKind::Button 
                            colour=Some(IconColour::Gray)
                            icon="close-outline".to_string() 
                            label="Cancel".to_string() 
                            on_click=on_click_cancel
                        />
                    </div>
                    <div class="flex flex-col">
                        <label class="rounded">"Url"</label>
                        <input type="text" on:input=on_input_url prop:value={ move || new_dispatcher_url.get()}/>

                        <div class="flex">
                            <div class="grow"/>
                            <IconButton 
                                kind=ButtonKind::Button 
                                colour=Some(IconColour::Blue)
                                icon="save-outline".to_string() 
                                label="Save".to_string() 
                                on_click=on_click_save
                            />
                        </div>
                    </div>
                    </div>
                    </div>
                </Show>
                <IconButton 
                    kind=ButtonKind::Button 
                    colour=Some(IconColour::Blue)
                    icon="add-outline".to_string() 
                    label="Add dispatcher".to_string() 
                    on_click=on_click_add
                />
            </div>
            <ul>
                <For
                    each={move || dispatchers.get().items}
                    key={|dispatcher| dispatcher.id }
                    view={move |cx, dispatcher| {
                        view! {
                            cx, 
                            <Messages dispatcher />
                        }
                    }}
                />
            </ul>
        </div>
    }

}
