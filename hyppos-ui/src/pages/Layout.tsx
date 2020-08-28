import React from "react";
import { Layout as AntLayout, Menu } from "antd";


export const Layout: React.FC = React.memo(
  function LoginPage({ children }) {
    return (
      <AntLayout>
        <AntLayout.Header style={{ position: "fixed", zIndex: 1, width: "100%" }}>
          <div className="logo"/>
          <Menu theme="dark" mode="horizontal" defaultSelectedKeys={["2"]}>
            <Menu.Item key="1">nav 1</Menu.Item>
            <Menu.Item key="2">nav 2</Menu.Item>
            <Menu.Item key="3">nav 3</Menu.Item>
          </Menu>
        </AntLayout.Header>
        <AntLayout.Content style={{ padding: "0 50px", marginTop: 64 }}>
          <div className="site-layout-background" style={{ padding: 24, minHeight: 380 }}>
            {children}
          </div>
        </AntLayout.Content>
        <AntLayout.Footer style={{ textAlign: "center" }}>Ant Design ©2018 Created by Ant UED</AntLayout.Footer>
      </AntLayout>
    )
  }
)
