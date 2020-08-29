import React, { PureComponent } from "react";
import styles from "./styles.module.css";
import { Card, Select } from 'antd';

export class ProjectsPage extends PureComponent {
  componentDidMount() {
    // gatewayClient.get("/projects").then(result => console.log(result));
  }

  state = {
    repos: [
      { link: '#', name: 'RustyRust', description: 'Seems very rusty', lang: 'rust' },
      { link: '#', name: 'Hello world', description: 'Undefined is not a function', lang: 'js' },
      { link: '#', name: 'Goodbye world', description: 'Koshka vasshe norm', lang: 'brainfuck' },
      { link: '#', name: 'RustyRust1', description: 'Seems very rusty', lang: 'rust' },
      { link: '#', name: 'Hello world1', description: 'Uncaught (in promise) Error: Request failed with status code', lang: 'js' },
      { link: '#', name: 'Goodbye world1', description: 'Koshka lomaet moi dom po kd', lang: 'go' },
      { link: '#', name: 'RustyRust2', description: 'There is no spoon', lang: 'rust' },
      { link: '#', name: 'Hello world2', description: 'Cannot update a component (`App`) while rendering a different component', lang: 'js' },
      { link: '#', name: 'Goodbye world2', description: 'Koshka beshenno krutit hvostom', lang: 'php' },
      { link: '#', name: 'RustyRust3', description: 'There is no spoon', lang: 'rust' },
      { link: '#', name: 'Hello world3', description: 'Uncaught ReferenceError: method is not defined', lang: 'js' },
      { link: '#', name: 'Goodbye world3', description: 'Koshka vasshe norm', lang: 'python' },
    ],
    selectedLang: 'all',

  };

  render() {
    const langs = this.state.repos.map(item => item.lang);

    return (<div className={styles.root}>
      <div className={styles.filter}>
        Выберите язык репозитория
        <Select defaultValue='all' style={{ width: 120, marginLeft: 10 }} onChange={this.handleChange}>
          <Select.Option value='all'>Все</Select.Option>
          {[...new Set(langs)].map(lang => (<Select.Option value={lang}>{lang}</Select.Option>))}
        </Select>
      </div>
      {this.renderRepos()}
    </div>)
  }

  handleChange = (lang: string) => {
    this.setState({
      selectedLang: lang,
    });
  };

  renderRepos = () => (
    this.state.repos.map(({ link, name, description, lang}) => {
        if (this.state.selectedLang === 'all' || this.state.selectedLang === lang)
        return (<div className={styles.repo}>
          <Card title={name} extra={<span className={styles.lang}>{lang}</span>} style={{ width: 300, height: 150 }}>
            <p>{description}</p>
          </Card>
          <a href={link} className={styles.review}>ПОРЕВЬЮВИТЬ</a>
        </div>)
}));
}
