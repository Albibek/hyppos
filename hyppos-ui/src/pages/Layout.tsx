import React from "react";
import { Col, Divider, Layout as AntLayout, Menu, Row } from "antd";
import classes from "./classes.module.scss";
import { ReactComponent as LogoIcon } from "../assets/logo.svg"
import { currentHistory } from "../history";

const { Header, Content, Footer } = AntLayout

export const Layout: React.FC = React.memo(
  function LoginPage({ children }) {
    const currentPath = currentHistory.location.pathname

    return (
      <AntLayout className={classes.root}>
        <Header className={classes.header}>
          <Row gutter={16}>
            <Col style={{ display: "flex", alignItems: "center" }}>
              <LogoIcon style={{ width: 25, height: 25, fill: "white" }}/>
            </Col>
            <Col flex={1}>
              <Menu theme="dark" mode="horizontal" defaultSelectedKeys={[currentPath]}>
                <Menu.Item key="/" onClick={() => currentHistory.push("/")}>Главная</Menu.Item>
                <Menu.Item key="/projects" onClick={() => currentHistory.push("/projects")}>Проекты</Menu.Item>
              </Menu>
            </Col>
          </Row>
        </Header>

        <Content className={classes.content}>
          <div className="site-layout-background" style={{ padding: 24, minHeight: 380 }}>
            {children}
          </div>
        </Content>

        <Footer style={{ textAlign: "center" }}>
          {/* eslint-disable-next-line react/no-unescaped-entities */}
          AvitoHack: Погружение ©2020, создано командой "Альтернативные Гиппопотамы"
        </Footer>
      </AntLayout>
    )
  }
)
