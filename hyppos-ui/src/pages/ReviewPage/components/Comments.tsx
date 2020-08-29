import React from "react";
import { Card, List, Typography } from "antd";
import { Comment } from "../types"
import { UserOutlined } from "@ant-design/icons";
import { observer } from "mobx-react-lite";


interface CommentsProps {
  comment: Comment
}

export const Comments = observer(
  function Comments({ comment }: CommentsProps) {
    return (
      <Card style={{ margin: "10px 15px" }} bodyStyle={{ padding: 0 }}>
        <List>
          <List.Item key={comment.id} style={{ padding: "10px 15px" }}>
            <List.Item.Meta
              avatar={<UserOutlined/>}
              title={<Typography.Text strong>{comment.user.name}</Typography.Text>}
              description={
                <Typography.Text>
                  {comment.message}
                </Typography.Text>
              }
            />
          </List.Item>
        </List>
      </Card>
    )
  }
)
