import { symbolDistance } from '@/utils';
import { Declaration, searchDeclaration } from './base';
import { SearchYield, SymbolYield, Containment } from './search';
import { Identifier, IdentifierWeights } from './types';
import { Argument } from './argument';

export interface TypeDefinition extends Declaration {
    tag: Identifier.TypeDefinition,

     /**
     * @brief Function signature
     * @readonly
     */
    readonly type: string,

    /**
     * @brief Parsed function signature
     * @readonly
     */
    readonly parsedSignature: TypeSignature,
}

export interface TypeSignature {
    /**
     * @brief Return type of the function
     * @readonly
     */
    readonly returnType: string,

    /**
     * @brief Arguments of the function
     * @readonly
     */
    readonly arguments: Argument[],
}

export interface TypeDefinitionYield extends SearchYield {
    readonly tag: Identifier.TypeDefinition,

    fields: TypeDefinition,
}
