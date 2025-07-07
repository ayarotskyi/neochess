import { type RefCallback, useCallback, useRef } from 'react';

export type DropCallback = (
  xUnits: number,
  yUnits: number,
  resetPosition: () => void,
) => void;

const useDraggable = (onDrop?: DropCallback) => {
  const targetElementRef = useRef<HTMLElement | null>(null);
  const draggableElementRef = useRef<HTMLElement | null>(null);

  const initialPosition = useRef<{ top: string; left: string }>({
    top: '0px',
    left: '0px',
  });

  const resetPosition = useCallback(() => {
    draggableElementRef.current?.style.setProperty(
      'top',
      initialPosition.current.top,
    );
    draggableElementRef.current?.style.setProperty(
      'left',
      initialPosition.current.left,
    );
  }, []);

  const onMouseDown = useCallback<React.MouseEventHandler<HTMLDivElement>>(
    (event) => {
      if (
        targetElementRef.current === null ||
        draggableElementRef.current === null
      ) {
        return;
      }

      event.preventDefault();

      if (event.button != 0) {
        return;
      }

      draggableElementRef.current!.style.setProperty(
        'top',
        event.clientY -
          targetElementRef.current!.offsetTop -
          draggableElementRef.current!.clientHeight / 2 +
          'px',
      );
      draggableElementRef.current!.style.setProperty(
        'left',
        event.clientX -
          targetElementRef.current!.offsetLeft -
          draggableElementRef.current!.clientWidth / 2 +
          'px',
      );
      const prevZIndex = draggableElementRef.current!.style.zIndex;
      draggableElementRef.current!.style.setProperty('z-index', '10');

      document.onmousemove = (event) => {
        event.preventDefault();
        draggableElementRef.current!.style.setProperty(
          'top',
          event.clientY -
            targetElementRef.current!.offsetTop -
            draggableElementRef.current!.clientHeight / 2 +
            'px',
        );
        draggableElementRef.current!.style.setProperty(
          'left',
          event.clientX -
            targetElementRef.current!.offsetLeft -
            draggableElementRef.current!.clientWidth / 2 +
            'px',
        );
      };
      document.onmouseup = (event) => {
        event.preventDefault();

        draggableElementRef.current!.style.setProperty('z-index', prevZIndex);
        onDrop?.(
          (event.clientX - targetElementRef.current!.offsetLeft) /
            draggableElementRef.current!.clientWidth,
          (event.clientY - targetElementRef.current!.offsetTop) /
            draggableElementRef.current!.clientHeight,
          resetPosition,
        );

        document.onmousemove = null;
        document.onmouseup = null;
      };
    },
    [onDrop, resetPosition],
  );

  const targetElementRefCallback = useCallback<RefCallback<HTMLElement | null>>(
    (ref) => {
      targetElementRef.current = ref;
    },
    [],
  );

  const draggableElementRefCallback = useCallback<
    RefCallback<HTMLElement | null>
  >((ref) => {
    draggableElementRef.current = ref;
    initialPosition.current = {
      top: ref?.style.top ?? '0px',
      left: ref?.style.left ?? '0px',
    };
  }, []);

  return {
    targetElementRefCallback,
    draggableElementRefCallback,
    onMouseDown,
  };
};

export default useDraggable;
