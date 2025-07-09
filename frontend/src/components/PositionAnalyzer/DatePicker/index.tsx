import { useAnalyzerStore } from '@/store/analyzer';
import {
  Button,
  type ButtonProps,
  Popover,
  Portal,
  Text,
} from '@chakra-ui/react';
import { useShallow } from 'zustand/shallow';
import CalendarComponent from './CalendarComponent';
import { DateObject } from 'react-multi-date-picker';
import { useCallback, useState } from 'react';

type Props = ButtonProps & {};

const DatePicker = (props: Props) => {
  const [open, setOpen] = useState(false);
  const onClose = useCallback(() => setOpen(false), []);

  const { fromUnix, toUnix } = useAnalyzerStore(
    useShallow((state) => state.timeRange),
  );

  return (
    <Popover.Root
      positioning={{
        placement: 'bottom-end',
      }}
      open={open}
      onOpenChange={(e) => setOpen(e.open)}
      unmountOnExit
      lazyMount
    >
      <Popover.Trigger asChild>
        <Button
          py="9px"
          px="24px"
          bg="rgba(17, 24, 39, 0.5)"
          border="1px solid rgba(6, 182, 212, 0.5)"
          boxShadow="0px 0px 10px rgba(0, 255, 255, 0.2)"
          _hover={{
            bg: 'rgba(31, 41, 55, 0.5)',
          }}
          borderRadius="0px"
          {...props}
        >
          <Text
            textStyle="mono"
            fontWeight={400}
            fontSize="12px"
            lineHeight="16px"
            color="#67E8F9"
          >
            {`${new DateObject(new Date(fromUnix * 1000)).format('MMM D, YYYY')} - ${new DateObject(new Date(toUnix * 1000)).format('MMM D, YYYY')}`}
          </Text>
        </Button>
      </Popover.Trigger>
      <Portal>
        <Popover.Positioner>
          <Popover.Content
            bg="rgba(17, 24, 39, 0.95)"
            border="1px solid rgba(6, 182, 212, 0.5)"
            boxShadow="0px 0px 40px rgba(0, 255, 255, 0.4)"
            backdropFilter="blur(6px)"
            borderRadius="12px"
            overflow="hidden"
            width="fit-content"
          >
            <Popover.Body p={0}>
              <CalendarComponent onClose={onClose} />
            </Popover.Body>
          </Popover.Content>
        </Popover.Positioner>
      </Portal>
    </Popover.Root>
  );
};

export default DatePicker;
