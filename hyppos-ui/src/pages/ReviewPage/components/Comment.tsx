import React from "react";
import { Button, Card, List, Typography } from "antd";
import { UserOutlined } from "@ant-design/icons";
import { observer } from "mobx-react-lite";
import TextArea from "antd/es/input/TextArea";


interface CommentProps {
  userName: string
  insertComment: (message: string) => void
  reset: () => void
}

export const CommentForm = observer(
  function CommentForm({ userName, insertComment, reset }: CommentProps) {
    const [state, setState] = React.useState<string>()

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
                    <TextArea value={state} onChange={e => setState(e.target.value)}/>
                  </Typography.Text>
                  <Button type="primary" onClick={() => {
                    if (state) {
                      insertComment(state)
                      reset()
                    }
                  }
                  }>добавить коммент</Button>
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
