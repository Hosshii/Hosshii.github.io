use yew::prelude::*;

#[function_component(Nav)]
pub fn nav_view() -> Html {
    html! {
    <nav
      class="navbar navbar-light navbar-expand-lg bg-light sticky-top"
      id="navbar"
    >
      <div class="container-fluid">
        <a class="navbar-brand" href="#"> {"Hosshi's Portfolio"} </a>
        <button
          class="navbar-toggler"
          type="button"
          data-bs-toggle="collapse"
          data-bs-target="#navbarNavAltMarkup"
          aria-controls="navbarNavAltMarkup"
          aria-expanded="false"
          aria-label="Toggle navigation"
        >
          <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
          <div class="navbar-nav ms-auto">
            <a class="nav-link" aria-current="page" href="#about"> {"about"} </a>
            <a class="nav-link" href="#skills"> {"skills"} </a>
            <a class="nav-link" href="#experiences"> {"experiences"} </a>
            <a class="nav-link" href="#works"> {"works"} </a>
          </div>
        </div>
      </div>
    </nav>
        }
}
