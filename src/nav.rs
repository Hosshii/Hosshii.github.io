use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav_view() -> Html {
    html! {
    <nav class="navbar navbar-expand-sm navbar-light bg-light">
      <div class="container-fluid">
        <Link<Route>
          classes={classes!("navbar-brand")} to={Route::Home}> {"Hosshi's
          Portfolio"} </Link<Route
        >>
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
            <Link<Route>
              classes={classes!("nav-link")} to={Route::About} >
              {"about"} </Link<Route
            >>
            <Link<Route>
              classes={classes!("nav-link")} to={Route::Skills} >
              {"skills"} </Link<Route
            >>
            <Link<Route>
              classes={classes!("nav-link")} to={Route::Experiences}>
              {"experiences"} </Link<Route
            >>
            <Link<Route>
              classes={classes!("nav-link")} to={Route::Works}>
              {"works"} </Link<Route
            >>
          </div>
        </div>
      </div>
    </nav>
    }
}
