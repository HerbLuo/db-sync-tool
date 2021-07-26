import React from "react";

interface Props extends React.InputHTMLAttributes<HTMLInputElement> {
  block?: boolean;
  label: string;
  wrap?: boolean;
  rowGrip?: number | string;
}

export const TextFieldDefProps: Partial<Props> = {
  block: true,
}

export function ZzTextField(props: Props) {
  const { 
    block, 
    label, 
    rowGrip, 
    wrap = false, 
    className, 
    ...others 
  } = {...TextFieldDefProps, ...props};

  const name = others.name || label;

  const childs = (
    <>
      <label htmlFor={name} className="zz-text-field-label" style={rowGrip ? {marginRight: rowGrip} : {}}>{props.label}</label>
      <input name={name} className={`zz-text-field-input ${className ?? ""}`} {...others}/>
    </>
  );
  return (
    wrap 
      ? (
        <div className={`zz-text-field ${block ? "block" : undefined}`}>
          {childs}
        </div> 
      )
      : childs
  );
}
