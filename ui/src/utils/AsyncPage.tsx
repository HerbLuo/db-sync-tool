import { RouteComponentProps } from "react-router";
import AsyncComponent from "./AsyncComponent";

function AsyncPage(props: RouteComponentProps<{ page: string }>) {
  const page = import(
      /* webpackChunkName: "[request]" */
      `../pages/${props.match.params.page}/index.tsx`
    );
    return (
      <AsyncComponent>
        {page.then(({ default: Comp }) => <Comp/>)}
      </AsyncComponent>
    );
}

export default AsyncPage;
