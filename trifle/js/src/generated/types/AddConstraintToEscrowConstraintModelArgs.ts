/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet';
import { EscrowConstraint, escrowConstraintBeet } from './EscrowConstraint';
export type AddConstraintToEscrowConstraintModelArgs = {
  constraintName: string;
  constraint: EscrowConstraint;
};

/**
 * @category userTypes
 * @category generated
 */
export const addConstraintToEscrowConstraintModelArgsBeet =
  new beet.FixableBeetArgsStruct<AddConstraintToEscrowConstraintModelArgs>(
    [
      ['constraintName', beet.utf8String],
      ['constraint', escrowConstraintBeet],
    ],
    'AddConstraintToEscrowConstraintModelArgs',
  );
