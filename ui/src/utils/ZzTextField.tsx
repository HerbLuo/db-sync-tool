import React from "react";

interface Props extends React.InputHTMLAttributes<HTMLInputElement> {
  block?: boolean;
  label: string;
}

export const TextFieldDefProps: Partial<Props> = {
  block: true,
}

export function ZzTextField(props: Props) {
  const { block, label, ...others } = {...TextFieldDefProps, ...props};

  return (
    <label className={`zz-text-field ${block ? "block" : undefined}`}>
      {props.label}
      <input {...others}/>
    </label>
  );
}
