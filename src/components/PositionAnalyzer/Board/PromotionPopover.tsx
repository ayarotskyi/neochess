import { useGameStore } from '@/store/game';
import { PieceComponents, type ColorRole } from '@/utils';
import { Button, Flex, HStack, VStack, type FlexProps } from '@chakra-ui/react';
import type { Role } from 'chessops';

export type PromotionCallback = (role: Role | undefined) => void;

type Props = FlexProps;

const promotionRoles: Role[][] = [
  ['queen', 'rook'],
  ['bishop', 'knight'],
];

const PromotionPopover = (props: Props) => {
  const promotingMove = useGameStore((state) => state.promotingMove);
  const playAs = useGameStore((state) => state.playAs);
  const resolvePromotion = useGameStore((state) => state.resolvePromotion);
  return (
    promotingMove && (
      <Flex
        position="absolute"
        w="100%"
        h="100%"
        onMouseDown={() => {
          resolvePromotion(null);
        }}
        zIndex={3}
        {...props}
      >
        <Flex
          width={0}
          height={0}
          overflow="visible"
          align="center"
          justify="center"
          position="absolute"
          top={
            100 *
              (playAs === 'white'
                ? 1 - ((1 / 8) * Math.floor(promotingMove.to / 8) + 1 / 16)
                : (1 / 8) * Math.floor(promotingMove.to / 8) + 1 / 16) +
            '%'
          }
          left={
            100 *
              (playAs === 'white'
                ? (1 / 8) * (promotingMove.to % 8) + 1 / 16
                : 1 - ((1 / 8) * (promotingMove.to % 8) + 1 / 16)) +
            '%'
          }
        >
          <VStack
            border="2px solid rgba(168, 85, 247, 0.5)"
            boxShadow="0px 0px 40px rgba(147, 51, 234, 0.6)"
            backdropFilter="blur(6px)"
            borderRadius="12px"
            spaceY="8px"
            p="18px"
            minW="fit-content"
            onMouseDown={(event) => event.stopPropagation()}
            position="absolute"
          >
            {promotionRoles.map((roles, index) => (
              <HStack key={index} spaceX="8px">
                {roles.map((role) => (
                  <Button
                    key={role}
                    bg="rgba(31, 41, 55, 0.5)"
                    border="1px solid rgba(168, 85, 247, 0.5)"
                    _hover={{
                      bg: 'rgba(147, 51, 234, 0.2)',
                    }}
                    borderRadius="8px"
                    px="12px"
                    py="8.5px"
                    onClick={() => resolvePromotion(role)}
                  >
                    {(() => {
                      const PieceComponent =
                        PieceComponents[
                          `${promotingMove.to > 55 ? 'white' : 'black'}_${role}` as ColorRole
                        ];
                      return <PieceComponent width="32px" height="32px" />;
                    })()}
                  </Button>
                ))}
              </HStack>
            ))}
          </VStack>
        </Flex>
      </Flex>
    )
  );
};

export default PromotionPopover;
