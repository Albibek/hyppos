import React, { PureComponent } from "react";
import styles from "./styles.module.css";
import { Card } from "antd";
import { observer } from "mobx-react-lite";
import { useProjectsPageStore } from "./store/ProjectsPageStore";
import { Link } from "react-router-dom";

const repos = [
  { link: "#", name: "RustyRust", description: "Seems very rusty", lang: "rust" },
  { link: "#", name: "Hello world", description: "Undefined is not a function", lang: "js" },
  { link: "#", name: "Goodbye world", description: "Koshka vasshe norm", lang: "brainfuck" },
  { link: "#", name: "RustyRust1", description: "Seems very rusty", lang: "rust" },
  {
    link: "#",
    name: "Hello world1",
    description: "Uncaught (in promise) Error: Request failed with status code",
    lang: "js"
  },
  { link: "#", name: "Goodbye world1", description: "Koshka lomaet moi dom po kd", lang: "brainfuck" },
  { link: "#", name: "RustyRust2", description: "There is no spoon", lang: "rust" },
  {
    link: "#",
    name: "Hello world2",
    description: "Cannot update a component (`App`) while rendering a different component",
    lang: "js"
  },
  { link: "#", name: "Goodbye world2", description: "Koshka beshenno krutit hvostom", lang: "brainfuck" },
  { link: "#", name: "RustyRust3", description: "There is no spoon", lang: "rust" },
  { link: "#", name: "Hello world3", description: "Uncaught ReferenceError: method is not defined", lang: "js" },
  { link: "#", name: "Goodbye world3", description: "Koshka vasshe norm", lang: "brainfuck" },
];


export const ProjectsPage = observer(
  function ProjectsPage() {
    const { data, fetchProjects, clear } = useProjectsPageStore()

    React.useEffect(() => {
      fetchProjects()

      return () => {
        clear()
      }
    }, [clear, fetchProjects])

    return <div className={styles.root}>
      {data && data.map(({ name, id, }) => (
        <div key={name} className={styles.repo}>
          <Card title={name} extra={<span className={styles.lang}>{"Rust"}</span>} style={{ width: 300, height: 150 }}>
            <p>{"Описание отсутствует"}</p>
          </Card>
          <Link to={`/projects/${name}+${id}/review`} className={styles.review}>ПОРЕВЬЮВИТЬ</Link>
        </div>)
      )}
    </div>
  }
)
