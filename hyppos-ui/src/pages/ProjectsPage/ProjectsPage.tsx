import React, { PureComponent } from "react";
import styles from "./styles.module.css";
import { Card } from 'antd';

const repos = [
  { link: '#', name: 'RustyRust', description: 'Seems very rusty', lang: 'rust' },
  { link: '#', name: 'Hello world', description: 'Undefined is not a function', lang: 'js' },
  { link: '#', name: 'Goodbye world', description: 'Koshka vasshe norm', lang: 'brainfuck' },
  { link: '#', name: 'RustyRust1', description: 'Seems very rusty', lang: 'rust' },
  { link: '#', name: 'Hello world1', description: 'Uncaught (in promise) Error: Request failed with status code', lang: 'js' },
  { link: '#', name: 'Goodbye world1', description: 'Koshka lomaet moi dom po kd', lang: 'brainfuck' },
  { link: '#', name: 'RustyRust2', description: 'There is no spoon', lang: 'rust' },
  { link: '#', name: 'Hello world2', description: 'Cannot update a component (`App`) while rendering a different component', lang: 'js' },
  { link: '#', name: 'Goodbye world2', description: 'Koshka beshenno krutit hvostom', lang: 'brainfuck' },
  { link: '#', name: 'RustyRust3', description: 'There is no spoon', lang: 'rust' },
  { link: '#', name: 'Hello world3', description: 'Uncaught ReferenceError: method is not defined', lang: 'js' },
  { link: '#', name: 'Goodbye world3', description: 'Koshka vasshe norm', lang: 'brainfuck' },
];

export class ProjectsPage extends PureComponent {
  componentDidMount() {
    // gatewayClient.get("/projects").then(result => console.log(result));
  }

  render() {
    return <div className={styles.root}>
      {repos.map(({ link, name, description, lang}) => (
        <div className={styles.repo}>
          <Card title={name} extra={<span className={styles.lang}>{lang}</span>} style={{ width: 300, height: 150 }}>
            <p>{description}</p>
          </Card>
          <a href={link} className={styles.review}>ПОРЕВЬЮВИТЬ</a>
        </div>)
      )}
    </div>
  }
}
