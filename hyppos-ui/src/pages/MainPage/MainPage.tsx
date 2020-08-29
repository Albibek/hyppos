import React, { PureComponent } from "react";
import { Card } from 'antd';
import styles from "../ProjectsPage/styles.module.css";

const mocks = [
  {
    "name": "react-saga-app",
    "owner": {
      "id": 10464533,
      "login": "Firefr0st"
    },
    "private": false,
    "fork": false,
    "description": "test react todo App with auth",
    "language": "JavaScript",
    "created_at": "2019-07-15T08:34:38Z",
    "updated_at": "2019-07-15T08:37:19Z"
  },
  {
    "name": "showDirTree",
    "owner": {
      "id": 10464533,
      "login": "Firefr0st"
    },
    "private": false,
    "fork": false,
    "description": "shows directory tree",
    "language": "Go",
    "created_at": "2020-05-10T20:25:19Z",
    "updated_at": "2020-05-10T20:30:23Z"
  }
];

export class MainPage extends PureComponent {
  render() {
    return (
      <div className={styles.root}>
        <h3>Мои репозитории</h3>

        {this.renderRepos()}
      </div>
    );
  }

  renderRepos = () => (
    mocks.map(({ name, description, language}) => {
        return (<div className={styles.repo}>
          <Card title={name} extra={<span className={styles.lang}>{language}</span>} style={{ width: 300, height: 150 }}>
            <p>{description}</p>
          </Card>
          <a href='#' className={styles.review}>Отправить на ревью</a>
        </div>)
    }));
}
