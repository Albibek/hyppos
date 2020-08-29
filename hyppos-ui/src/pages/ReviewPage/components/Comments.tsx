import React from "react";
import { Card, List, Typography } from "antd";
import { Comment } from "../types"
import { UserOutlined } from "@ant-design/icons";


interface CommentsProps {
  comments: Comment[]
}

export const Comments = ({ comments }: CommentsProps) => {
  return (
    <Card style={{ margin: "10px 15px" }} bodyStyle={{ padding: 0 }}>
      <List>
        {comments.map(comment => {
          return (
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
          )
        })}
      </List>
    </Card>
  )
}
