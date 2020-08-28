import React from "react";
import { Button, Card, Layout } from "antd";
import classes from "./classes.module.scss";
import { ReactComponent as LogoIcon } from "../../assets/logo.svg";
import { GithubOutlined } from "@ant-design/icons/lib";
import { currentHistory } from "../../history";
import { useLocation } from "react-router";
import { config } from "../../config";
import { api } from "../../api";

export const LoginPage = React.memo(
  function LoginPage() {
    const location = useLocation()

    const redirectToOAuth = () => {
      // add current location duplicate to browser history. This is a hack to use "Back" button correctly
      currentHistory.push(location)

      window.location.replace(`${config.gatewayUrl}/auth/login`)
    }

    api.logout().then(res => {
      console.log(res)
    })

    return (
      <Layout className={classes.root}>
        <Card className={classes.card}>
          <div className={classes.logoWrapper}>

            <LogoIcon style={{ width: 75, height: 75 }}/>
          </div>

          <p>
            Сервис для публичного Code Review. Смотри, не расплачься!
          </p>


          <Button
            className={classes.action}
            icon={<GithubOutlined/>}
            type="primary"
            onClick={redirectToOAuth}
            // loading={loading}
          >
            Войти через GitHub
          </Button>
        </Card>
      </Layout>
    )
  }
)
