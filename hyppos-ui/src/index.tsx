import React from "react";
import ReactDOM from "react-dom";
import * as serviceWorker from "./serviceWorker";
import { Redirect, Route, Router, Switch } from "react-router";
import { createBrowserHistory } from "history"
import { LoginPage } from "./pages/LoginPage/LoginPage";
import { ReviewPage } from "./pages/ReviewPage/ReviewPage";
import { ProjectsPage } from "./pages/ProjectsPage/ProjectsPage";
import { NotFoundPage } from "./pages/NotFoundPage/NotFoundPage";
import { Layout } from "./pages/Layout";

const history = createBrowserHistory()

const App = React.memo(
  function App() {
    return (
      <Router history={history}>
        <Switch>
          <Route path="/login" exact component={LoginPage}/>

          <Layout>
            <Route path="/projects" exact component={ProjectsPage}/>
            <Route path="/review/:projectName" exact component={ReviewPage}/>
          </Layout>

          <Redirect exact from="/" to="/projects"/>
          <Route component={NotFoundPage}/>
        </Switch>
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
