/// <reference types="react-scripts" />
declare module "*.svg" {
  const ReactComponent: unknown
  export { ReactComponent }
}

declare module "*.module.scss" {
  const classes: { [key: string]: string };
  export default classes
}
