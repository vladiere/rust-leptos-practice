use charts_rs::*;
use leptos::*;
use leptos::{leptos_dom::logging::console_log, wasm_bindgen::JsCast};
use wasm_bindgen::JsValue;
use web_sys::js_sys::JsString;
use web_sys::{window, Document, DomParser, Element, SupportedType};

#[component]
pub fn BarChart() -> impl IntoView {
    let bar_chart = BarChart::from_json(
        r###"{
  "border_radius": 8,
  "font_family": "Roboto",
  "height": 400,
  "inner_radius": 30,
  "legend_margin": {
    "top": 50
  },
  "legend_show": true,
  "radius": 110,
  "rose_type": true,
  "series_list": [
    {
      "name": "rose 1",
      "data": [
        40
      ]
    },
    {
      "name": "rose 2",
      "data": [
        38
      ]
    },
    {
      "name": "rose 3",
      "data": [
        32
      ]
    },
    {
      "name": "rose 4",
      "data": [
        30
      ]
    },
    {
      "name": "rose 5",
      "data": [
        28
      ]
    },
    {
      "name": "rose 6",
      "data": [
        26
      ]
    },
    {
      "name": "rose 7",
      "data": [
        22
      ]
    },
    {
      "name": "rose 8",
      "data": [
        18
      ]
    }
  ],
  "sub_title_text": "Sub Title",
  "theme": "grafana",
  "title_text": "Nightingale Chart",
  "type": "pie",
  "width": 600
}"###,
    )
    .unwrap();

    let parser = DomParser::new().unwrap();
    let svg_string = bar_chart.svg().unwrap();

    let document = parser
        .parse_from_string(&svg_string, SupportedType::ImageSvgXml)
        .unwrap();
    let svg_element = document
        .document_element()
        .expect("Failed to get document element")
        .dyn_into::<Element>()
        .expect("Failed to cast to Element");

    // Get a reference to the target element in the DOM
    let target_element = if let Some(window) = window() {
        if let Some(document) = window.document() {
            document.get_element_by_id("barchart")
        } else {
            None
        }
    } else {
        None
    };

    if let Some(target_element) = target_element {
        target_element
            .append_child(&svg_element)
            .expect("Failed to append SVG element to target element");
    }

    view! {
        <div>{ bar_chart.svg().unwrap() }</div>
        <div id="barchart"></div>
    }
}
