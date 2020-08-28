import React from "react";

export const Layout: React.FC = React.memo(
  function LoginPage({ children }) {
    return (
      <div>
        {children}
      </div>
    )
  }
)
