import { useAnalyzerStore } from '@/store/analyzer';
import { Button, Flex, HStack, type FlexProps } from '@chakra-ui/react';
import { useCallback, useState } from 'react';
import {
  Calendar,
  DateObject,
  type CalendarProps,
} from 'react-multi-date-picker';
import './calendar.css';
import LeftArrowIcon from '@/icons/LeftArrowIcon';

type Props = FlexProps & { onClose: () => void };

const weekDays = ['S', 'M', 'T', 'W', 'T', 'F', 'S'];

const renderButton = (
  direction: 'left' | 'right',
  handleClick: () => void,
  disabled: boolean,
) => {
  return (
    <Button
      onClick={handleClick}
      _icon={{
        width: '16px',
        height: '16px',
      }}
      disabled={disabled}
      _disabled={{
        cursor: 'default',
      }}
      bg="transparent"
      _hover={{
        bg: 'rgb(6 182 212 / 0.2)',
      }}
      p="8px"
      borderRadius="8px"
      height="fit-content"
      minWidth="0px"
    >
      <LeftArrowIcon
        style={{
          transform: `${direction === 'right' ? 'rotate(180deg)' : undefined}`,
        }}
        color="#22D3EE"
      />
    </Button>
  );
};

const mapDays: CalendarProps['mapDays'] = () => {
  return {
    style: {
      borderRadius: '8px',
      fontFamily: 'Geist Mono Variable',
      fontWeight: 400,
      fontSize: '12px',
      lineHeight: '16px',
      color: '#67E8F9',
      width: '100%',
      height: '100%',
    },
  };
};

const currentDate = new DateObject();

const CalendarComponent = ({ onClose, ...props }: Props) => {
  const setTimeRange = useAnalyzerStore((state) => state.setTimeRange);

  const [values, setValues] = useState(
    (() => {
      const state = useAnalyzerStore.getState();
      return [
        new DateObject(state.timeRange.fromUnix * 1000),
        new DateObject(state.timeRange.toUnix * 1000),
      ];
    })(),
  );

  const onChange = useCallback<
    (selectedDates: DateObject | DateObject[] | null) => void
  >((selectedDates) => {
    if (selectedDates === null) {
      return;
    }

    if (selectedDates instanceof DateObject) {
      setValues([selectedDates]);
      return;
    }

    setValues(selectedDates);
  }, []);

  const apply = useCallback(() => {
    setTimeRange({
      fromUnix: values[0].unix,
      toUnix: values[1].unix,
    });
    onClose();
  }, [onClose, setTimeRange, values]);

  return (
    <Flex direction="column" {...props}>
      <Calendar
        value={values}
        onChange={onChange}
        renderButton={renderButton}
        weekDays={weekDays}
        mapDays={mapDays}
        range
        rangeHover
        maxDate={currentDate}
        monthYearSeparator="&#x200B;"
      />
      <HStack
        justify="space-between"
        p="16px"
        borderTop="1px solid rgba(6, 182, 212, 0.3)"
      >
        <Button
          bg="rgba(55, 65, 81, 0.5)"
          border="1px solid rgba(75, 85, 99, 0.5)"
          borderRadius="8px"
          fontFamily="Geist Mono Variable"
          fontWeight={400}
          fontSize="14px"
          lineHeight="20px"
          _hover={{
            bg: 'rgb(75 85 99 / 0.5)',
          }}
          onClick={onClose}
        >
          Cancel
        </Button>
        <Button
          bg="linear-gradient(90deg, #0891B2 0%, #06B6D4 100%)"
          border="1px solid rgba(34, 211, 238, 0.5)"
          boxShadow="0px 0px 15px rgba(0, 255, 255, 0.4)"
          borderRadius="8px"
          fontFamily="Geist Mono Variable"
          fontWeight={400}
          fontSize="14px"
          lineHeight="20px"
          onClick={apply}
          _hover={{
            boxShadow: '0px 0px 20px rgba(0,255,255,0.6)',
            bg: 'linear-gradient(90deg, oklch(71.5% .143 215.221) 0%, oklch(78.9% .154 211.53) 100%)',
          }}
        >
          Apply
        </Button>
      </HStack>
    </Flex>
  );
};

export default CalendarComponent;
