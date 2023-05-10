use crate::components::common::{IconGitHub, PageTitle};
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
    <>
      <div class="container mt-2 mx-2">
        <PageTitle> { "About Me" } </PageTitle>

        <div class="d-flex flex-row align-items-start">
          <div>
            <img
              src="https://github.com/Hosshii.png"
              class="rounded-circle"
              style="max-width: 100px"
            />
          </div>
          <div class="container">
            <div class="row">
              <div class="col col-md-auto">{ "HN: Hosshii" }</div>
            </div>
            <div class="row">
              <div class="col-md-auto">
                { "卒業: 東京工業大学 工学院 情報通信系 学士" }
              </div>
            </div>
            <p>
              {
              "大学のサークルでプログラミングを始め、バックエンドや低レイヤーなどをやっています。ものの仕組みを知るのが好きです。"
              }
            </p>
            <ul class="list-group list-group-flush">
              <li class="list-group-item" style="border-width: 0px">
                <IconGitHub />
                { "GitHub: " }
                <a href="https://github.com/Hosshii"> { "@Hosshii" }</a>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </>
          }
}
