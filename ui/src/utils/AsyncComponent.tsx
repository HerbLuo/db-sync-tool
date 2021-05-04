/**
 * version 190828
 */
import * as React from "react";

type ReactNodeOrArray = React.ReactNode | React.ReactNode[];

type ReactClassType<P = {}> = (new(props: P) => React.Component<P>);
type GetProps<T extends ReactClassType> = T extends ReactClassType<infer P> ? P : never;

interface Props<C extends ReactClassType> {
  children?: ReactNodeOrArray | Promise<ReactNodeOrArray>;
  element?: ReactNodeOrArray | Promise<ReactNodeOrArray>;
  component?: C | Promise<{ default: C }>;
  loading?: ReactNodeOrArray;
}

interface State {
  elements: ReactNodeOrArray | null;
}

const isPromise = (obj: any): obj is Promise<any> => obj && obj.then;

/**
 * usage:
 *
 * <AsyncComponent
 *   element={Promise.resolve(1).then(r => (<div>{r}</div>))}
 * />
 */
class AsyncComponent<C extends ReactClassType<any>>
  extends React.PureComponent<GetProps<C> & Props<C>, State> {
  private static renderOne(
    element: React.ReactNode,
    className: string,
    style: React.CSSProperties,
    others: any,
  ): React.ReactNode {
    if (!React.isValidElement(element)) {
      return element;
    }
    const props = element.props as Record<string, any>;
    const newProps = Object.assign({}, props, {
      className: (props.className || "") + " " + (className || ""),
      style: Object.assign({}, props.style, style),
      ...others,
    });
    return React.cloneElement(element, newProps as any);
  }


  public state: State = {
    elements: null,
  };

  private unmounted = false;
  private loadingElement = <div style={this.props.style}>loading</div>;
  private errorElement = <div style={this.props.style}>error</div>;

  /**
   * 当 props 改变后，等待 promise完成后重新计算，state
   */
  public async reComputeElement() {
    const { element, children, component } = this.props;
    let elements: ReactNodeOrArray;
    try {
      if (component) {
        const Comp = isPromise(component)
          ? (await component).default
          : component;
        elements = <Comp/>;
      } else {
        elements = await (element || children);
      }
    } catch (e) {
      console.error(e);
      elements = this.errorElement;
    }

    if (this.unmounted) {
      return;
    }
    this.setState({
      elements,
    });
  }

  public componentDidMount(): void {
    this.reComputeElement().catch(console.error);
  }

  public componentDidUpdate({ children, element }: GetProps<C> & Props<C>) {
    if (this.props.children !== children || this.props.element !== element) {
      this.reComputeElement().catch(console.error);
    }
  }

  public componentWillUnmount(): void {
    this.unmounted = true;
  }

  public render() {
    const elementOrNull = this.state.elements;
    if (!elementOrNull) {
      if (this.props.loading !== undefined) {
        return this.props.loading as any;
      }
      return this.loadingElement;
    }
    if (elementOrNull instanceof Array) {
      return this.renderAsElements(elementOrNull);
    }
    return this.renderAsElement(elementOrNull);
  }

  /**
   * 作为列表渲染
   * @param elements
   */
  private renderAsElements(elements: React.ReactNode[]): React.ReactNode[] {
    const { className, style, children, ...others } = this.props as Record<string, any>;
    return elements.map(element =>
      AsyncComponent.renderOne(element, className, style, others));
  }

  /**
   * 作为单个组件渲染
   * @param element
   */
  private renderAsElement(
    element: React.ReactNode,
  ): React.ReactNode {
    const { className, style, children, ...others } = this.props as Record<string, any>;
    return AsyncComponent.renderOne(element, className, style, others);
  }
}

export default AsyncComponent;
