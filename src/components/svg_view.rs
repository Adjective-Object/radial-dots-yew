use crate::svg::svg_drawable::SvgRenderer;

use yew::{html, Html};

pub fn svg_data_url<TStyle>(svg_renderer: &SvgRenderer<TStyle>, style: &TStyle) -> String {
    let raw_svg_string: String = svg_renderer.as_standalone_svg(style);
    let img_base64_src: String = format!(
        "data:image/svg+xml;base64,{}",
        base64::encode_config(&raw_svg_string, base64::STANDARD)
    );
    return img_base64_src;
}

pub fn svg_view<T: yew::html::Component, TStyle>(
    svg_renderer: &SvgRenderer<TStyle>,
    style: &TStyle,
) -> Html<T> {
    let img_base64_src = svg_data_url(svg_renderer, style);

    return html! {
        <img class="preview-image", src=img_base64_src, />
    };
}
