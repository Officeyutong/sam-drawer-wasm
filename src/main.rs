use layout::{
    backends::svg::SVGWriter,
    gv::{DotParser, GraphBuilder},
};
use sam::SAMPool;
use web_sys::{wasm_bindgen::JsCast, EventTarget, HtmlInputElement};
use yew::{function_component, html, use_state, Callback, Event, Html, Properties};

mod sam;

mod console {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace=console)]
        pub fn log(s: &str);
    }
}
fn generate_svg(text: impl AsRef<str>) -> String {
    let out_text = {
        let mut pool = SAMPool::default();
        for (id, s) in text.as_ref().split("|").enumerate() {
            pool.join_string(s, id as i32);
        }
        pool.collect();
        String::from_utf8(pool.generate_graph()).unwrap()
    };
    let parsed = {
        let mut parser = DotParser::new(&out_text);
        parser.process().unwrap()
    };
    let mut vg = {
        let mut builder = GraphBuilder::new();
        builder.visit_graph(&parsed);
        builder.get()
    };
    let mut svg = SVGWriter::new();
    
    vg.do_it(false, false, false, &mut svg);
    svg.finalize()
}

#[derive(Properties, PartialEq)]
pub struct SAMRenderProperties {
    sam_string: String,
}
#[function_component]
fn SAMRender(props: &SAMRenderProperties) -> Html {
    Html::from_html_unchecked(props.sam_string.clone().into())
}
#[function_component]
fn App() -> Html {
    let error_message = use_state(|| Option::<String>::None);
    let sam_string = use_state(|| String::default());
    let loading = use_state(|| false);
    let result_svg = use_state(|| Option::<String>::None);
    let type_callback = Callback::from({
        let sam_string = sam_string.clone();
        move |evt: Event| {
            let target: Option<EventTarget> = evt.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                sam_string.set(input.value());
            }
        }
    });
    let on_submit = {
        let sam_string = sam_string.clone();
        let result_svg = result_svg.clone();
        move |_| {
            let result = generate_svg(&*sam_string);
            console::log(&format!("dot result: {}", result));
            result_svg.set(Some(result));
        }
    };
    html! {
        <div class={"ui main container"} style={"margin-top:70px;margin-bottom:70px"}>
            <div style="top: 10%">
                <div class={"ui left aligned container"} style={"width: 100%"}>
                    <div class={"ui header"}>
                        <h1>{"SAM Drawer"}</h1>
                    </div>
                    if let Some(ref error_message) = *error_message {
                        <div class={"ui error message"}>
                            <div class={"ui header"}>
                                {"错误"}
                            </div>
                            <p>{error_message}</p>
                        </div>
                    }
                    <div class={"ui segment stacked"}>
                        if *loading {
                            <div class={"ui inverted active dimmer"}>
                                <div class={"ui text loader"}>
                                    {"正在加载.."}
                                </div>
                            </div>
                        }
                        <div class={"ui form"}>
                            <div class={"field"}>
                                <label>{"请输入字符串:"}</label>
                                <input type={"text"} id={"string"} value={(*sam_string).clone()} onchange={type_callback} />
                            </div>
                            <p>{"如果要绘制广义SAM,请使用|分隔字符串。绘制时可能会卡住，请耐心等待。本应用不会将你的数据发送给任何第三方。"}</p>
                            <div class={"ui submit button"} id={"submit-button"} onclick={on_submit}>{"提交"}</div>
                        </div>
                    </div>
                    if let Some(ref result) = *result_svg {
                        <div class={"ui segment stacked"}>
                            if *loading {
                                <div class={"ui inverted active dimmer"}>
                                    <div class={"ui text loader"}>
                                        {"正在加载.."}
                                    </div>
                                </div>
                            }
                            <div class={"ui center aligned container"} style={"min-height: 100px;overflow-x: scroll;overflow-y: scroll;"}>
                                <SAMRender ~sam_string={result.clone()}></SAMRender>
                            </div>
                        </div>
                    }
                </div>
            </div>
            <div class={"ui center aligned container"} style={"top: 50px;position: relative;"}>
                <div style={"color: darkgrey"}>
                    {"Made by MikuNotFoundException"}<br />
                    <a href={"https://github.com/Officeyutong/sam-drawer-wasm"}>{"Github"}</a>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
