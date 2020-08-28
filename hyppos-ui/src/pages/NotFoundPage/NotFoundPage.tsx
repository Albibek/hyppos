import React from "react";
import { Result } from "antd";
import { QuestionOutlined } from "@ant-design/icons/lib";

export const NotFoundPage = React.memo(
  function NotFoundPage() {

    return (
      <Result
        status="warning"
        icon={<QuestionOutlined translate />}
        title="404"
        style={{ position: "absolute", top: "25%", left: 0, width: "100%" }}
        subTitle="страница не существует"
      />
    )
  }
)
