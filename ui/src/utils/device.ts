export function areDesktop(): boolean {
  return document.body.clientWidth > 960
}

export type WidthRange = "xsUp" | "smUp" | "mdUp" | "lgUp" | "xlUp" |
  "xsDown" | "smDown" | "mdDown" | "lgDown" | "xlDown";

export const sizeQueryMap: Record<WidthRange, (size: number) => boolean> = {
  "xsUp": size => size > 0,
  "smUp": size => size > 600,
  "mdUp": size => size > 960,
  "lgUp": size => size > 1280,
  "xlUp": size => size > 1920,
  "xsDown": size => size < 600,
  "smDown": size => size < 960,
  "mdDown": size => size < 1280,
  "lgDown": size => size < 1920,
  "xlDown": _ => true,
};

// export function useBreakpoints<T>(breakpoints: Partial<Record<WidthRange, T[]>>): T[] {
//   const querySize = () => {
//     const size = document.body.clientWidth;
//     const objects: T[] = [];
//     for (const [key, value] of Object.entries(breakpoints) as Array<[WidthRange, T[] | undefined]>) {
//       if (sizeQueryMap[key](size) && value) {
//         objects.push(...value)
//       }
//     }
//     return objects;
//   };
//   const [objects, setObjects] = React.useState(querySize());
//   React.useEffect(() => {
//     let active = true;
//     const listener = debounce(() => {
//       if (active) {
//         setObjects(querySize())
//       }
//     }, 200);
//     window.addEventListener("resize", listener);
//     return () => {
//       active = false;
//       window.removeEventListener("resize", listener);
//     }
//   });
//   return objects;
// }
