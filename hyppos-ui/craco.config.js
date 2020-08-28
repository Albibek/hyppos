/* eslint-disable */

// плагин для изменения темы antd. Настройки меняются через global.theme.less.
//  Подробнее тут - https://ant.design/docs/react/use-with-create-react-app
const CracoLessPlugin = require("craco-less");

module.exports = {
  plugins: [
    {
      plugin: CracoLessPlugin,
      options: {
        lessLoaderOptions: {
          javascriptEnabled: true,
        },
      },
    },
  ],
};
