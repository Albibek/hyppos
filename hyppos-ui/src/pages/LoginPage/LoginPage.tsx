import React from "react";
import { Button, Card, Layout } from "antd";
import classes from "./classes.module.scss";
import { ReactComponent as LogoIcon } from "../../assets/logo.svg";
import { GithubOutlined } from "@ant-design/icons/lib";
import { currentHistory } from "../../history";
import { Redirect, useLocation } from "react-router";
import { config } from "../../config";
import { observer } from "mobx-react-lite";
import { useRootStore } from "../../store/RootStore";
import { api } from "../../api";

export const LoginPage = observer(
  function LoginPage() {
    const { authStore } = useRootStore()
    const location = useLocation()

    const redirectToOAuth = () => {
      // add current location duplicate to browser history. This is a hack to use "Back" button correctly
      currentHistory.push(location)

      window.location.replace(`${config.gatewayUrl}/auth/login`)
    }

    const userId = new URLSearchParams(location.search).get("userId")
    const userName = new URLSearchParams(location.search).get("userLogin")

    if (location.pathname === "/oauthCallback" && userName && userId) {
      authStore.login(userId, userName)

      api.insertProject({ externalId: 290972976 })

      return <Redirect to="/"/>
    }


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
