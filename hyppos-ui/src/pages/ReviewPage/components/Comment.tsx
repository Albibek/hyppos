import React from "react";
import { Button, Card, List, Typography } from "antd";
import { Comment } from "../types"
import { UserOutlined } from "@ant-design/icons";
import { observer } from "mobx-react-lite";
import TextArea from "antd/es/input/TextArea";


interface CommentProps {
  userName: string
  reset: () => void
}

export const CommentForm = observer(
  function CommentForm({ userName, reset }: CommentProps) {
    return (
      <Card style={{ margin: "10px 15px" }} bodyStyle={{ padding: 0 }}>
        <List>
          <List.Item style={{ padding: "10px 15px" }}>
            <List.Item.Meta
              avatar={<UserOutlined/>}
              title={<Typography.Text strong>{userName}</Typography.Text>}
              description={
                <div>
                  <Typography.Text>
                    <TextArea/>
                  </Typography.Text>
                  <Button type="link" onClick={reset}>отмена</Button>
                </div>
              }
            />
          </List.Item>
        </List>
      </Card>
    )
  }
)
