import React from "react";
import ReactDOM from "react-dom";
import * as serviceWorker from "./serviceWorker";
import { Redirect, Route, Router, Switch } from "react-router";
import { currentHistory } from "./history"
import { observer } from "mobx-react-lite";
import { useRootStore } from "./store/RootStore";

// styles
import "./global.scss"
import "./global.theme.less"

// pages
import { LoginPage } from "./pages/LoginPage/LoginPage";
import { ReviewPage } from "./pages/ReviewPage/ReviewPage";
import { ProjectsPage } from "./pages/ProjectsPage/ProjectsPage";
import { NotFoundPage } from "./pages/NotFoundPage/NotFoundPage";
import { Layout } from "./pages/Layout";
import { MainPage } from "./pages/MainPage/MainPage";


const App = observer(
  function App() {
    const { isAppLoading, authStore } = useRootStore()

    return (
      <Router history={currentHistory}>
        {isAppLoading ? (
          <span>приложение загружается...</span>
        ) : (
          <Switch>
            <Route path="/login" exact={true} component={LoginPage}/>
            <Route path="/404" exact={true} component={NotFoundPage}/>

            <Layout>
              <Switch>
                {authStore.isLoggedIn ? (
                  <>
                    <Route path="/" exact={true} component={MainPage}/>
                    <Route path="/projects" exact={true} component={ProjectsPage}/>
                    <Route path="/projects/:projectName/review" exact={true} component={ReviewPage}/>
                  </>
                ) : (
                  <Redirect to="/login"/>
                )}

                <Redirect to="/404"/>
              </Switch>
            </Layout>
          </Switch>
        )}
      </Router>
    )
  }
)

ReactDOM.render(
  <React.StrictMode>
    <App/>
  </React.StrictMode>,
  document.getElementById("root")
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
