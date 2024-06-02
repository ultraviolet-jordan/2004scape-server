import LocType from '#lostcity/cache/LocType.js';
import ParamType from '#lostcity/cache/ParamType.js';
import NpcType from '#lostcity/cache/NpcType.js';
import HuntType from '#lostcity/cache/HuntType.js';
import NpcMode from '#lostcity/entity/NpcMode.js';
import SpotanimType from '#lostcity/cache/SpotanimType.js';
import EnumType from '#lostcity/cache/EnumType.js';
import ObjType from '#lostcity/cache/ObjType.js';
import {Inventory} from '#lostcity/engine/Inventory.js';
import InvType from '#lostcity/cache/InvType.js';
import CategoryType from '#lostcity/cache/CategoryType.js';
import IdkType from '#lostcity/cache/IdkType.js';
import HuntVis from '#lostcity/entity/hunt/HuntVis.js';
import {LocAngle, LocShape} from '@2004scape/rsmod-pathfinder';
import {Position} from '#lostcity/entity/Position.js';
import {ConfigType} from '#lostcity/cache/ConfigType.js';
import SeqType from '#lostcity/cache/SeqType.js';
import VarPlayerType from '#lostcity/cache/VarPlayerType.js';
import VarNpcType from '#lostcity/cache/VarNpcType.js';
import VarSharedType from '#lostcity/cache/VarSharedType.js';
import FontType from '#lostcity/cache/FontType.js';
import MesanimType from '#lostcity/cache/MesanimType.js';
import StructType from '#lostcity/cache/StructType.js';
import DbRowType from '#lostcity/cache/DbRowType.js';
import DbTableType from '#lostcity/cache/DbTableType.js';

interface ScriptValidator<T, R> {
    validate(input: T): R;
}

class ScriptInputNumberNotNullValidator implements ScriptValidator<number, number> {
    validate(input: number): number {
        if (input !== -1) {
            return input;
        }
        throw Error('An input number was null(-1).');
    }
}

class ScriptInputStringNotNullValidator implements ScriptValidator<string, string> {
    validate(input: string): string {
        if (input.length > 0) {
            return input;
        }
        throw Error('An input string was null(-1).');
    }
}

class ScriptInputConfigTypeValidator<T extends ConfigType> implements ScriptValidator<number, T> {
    private readonly type: (input: number) => T | undefined;
    private readonly name: string;

    constructor(type: (input: number) => T | undefined, name: string) {
        this.type = type;
        this.name = name;
    }

    validate(input: number): T {
        const type: T | undefined = this.type(input);
        if (type) {
            return type;
        }
        throw new Error(`An input for a ${this.name} type was not valid to use. Input was ${input}.`);
    }
}

class ScriptInputLocAngleValidator implements ScriptValidator<number, LocAngle> {
    validate(input: number): LocAngle {
        if (input >= LocAngle.WEST && input <= LocAngle.SOUTH) {
            return input;
        }
        throw new Error(`An input for a Loc angle was out of range. Range should be: ${LocAngle.WEST} to ${LocAngle.SOUTH}.`);
    }
}

class ScriptInputLocShapeValidator implements ScriptValidator<number, LocShape> {
    validate(input: number): LocShape {
        if (input >= LocShape.WALL_STRAIGHT && input <= LocShape.GROUND_DECOR) {
            return input;
        }
        throw new Error(`An input for a Loc shape was out of range. Range should be: ${LocShape.WALL_STRAIGHT} to ${LocShape.GROUND_DECOR}.`);
    }
}

class ScriptInputDurationValidator implements ScriptValidator<number, number> {
    validate(input: number): number {
        if (input > 0) {
            return input;
        }
        throw new Error(`An input duration was out of range. Range should be greater than 0. Input was ${input}.`);
    }
}

class ScriptInputCoordValidator implements ScriptValidator<number, Position> {
    validate(input: number): Position {
        if (input >= 0 && input <= 0x3ffffffffff) {
            return Position.unpackCoord(input);
        }
        throw new Error(`An input coord was out of range. Range should be: 0 to 4398046511103. Input was ${input}.`);
    }
}

class ScriptInputNpcStatValidator implements ScriptValidator<number, number> {
    validate(input: number): number {
        if (input >= 0 && input < 6) {
            return input;
        }
        throw new Error(`An input for a Npc stat was not in a valid range. Range should be: 0 to 5. Input was ${input}.`);
    }
}

class ScriptInputQueueValidator implements ScriptValidator<number, number> {
    validate(input: number): number {
        if (input >= 0 && input < 20) {
            return input;
        }
        throw new Error(`An input for an ai_queue was not in a valid range. Range should be: 0 to 19. Input was ${input}.`);
    }
}

class ScriptInputNpcModeValidator implements ScriptValidator<number, number> {
    validate(input: number): number {
        if (input >= NpcMode.NULL && input <= NpcMode.APNPC5) {
            return input;
        }
        throw new Error(`An input for a Npc mode was not in a valid range. Range should be ${NpcMode.NULL} to ${NpcMode.APNPC5}.`);
    }
}

class ScriptInputHitTypeValidator implements ScriptValidator<number, number> {
    validate(input: number): number {
        if (input >= 0 && input <= 2) {
            return input;
        }
        throw new Error(`An input for a hit type was not in a valid range. Range should be 0 to 2. Input was ${input}.`);
    }
}

class ScriptInputObjCountValidator implements ScriptValidator<number, number> {
    validate(input: number): number {
        if (input > 0 && input <= Inventory.STACK_LIMIT) {
            return input;
        }
        throw new Error(`An input for an Obj count was not in a valid range. Range should be: 1 to ${Inventory.STACK_LIMIT}. Input was ${input}.`);
    }
}

class ScriptInputObjNotDummyValidator implements ScriptValidator<number, number> {
    validate(input: number): number {
        const type: ObjType | undefined = ObjType.get(input);
        if (type && type.dummyitem === 0) {
            return input;
        }
        throw new Error(`An input for an Obj was a graphic_only dummyitem. Input was ${input}.`);
    }
}

class ScriptInputHuntVisValidator implements ScriptValidator<number, HuntVis> {
    validate(input: number): HuntVis {
        if (input >= HuntVis.OFF && input <= HuntVis.LINEOFWALK) {
            return input;
        }
        throw new Error(`An input for a a hunt vis was not in a valid range. Range should be: ${HuntVis.OFF} to ${HuntVis.LINEOFWALK}.`);
    }
}

class ScriptInputFontTypeValidator implements ScriptValidator<number, FontType> {
    validate(input: number): FontType {
        const type: FontType | undefined = FontType.get(input);
        if (type) {
            return type;
        }
        throw new Error(`An input for a Font type was not valid to use. Input was ${input}.`);
    }
}

export const NumberNotNull: ScriptValidator<number, number> = new ScriptInputNumberNotNullValidator();
export const StringNotNull: ScriptValidator<string, string> = new ScriptInputStringNotNullValidator();
export const LocTypeValid: ScriptValidator<number, LocType> = new ScriptInputConfigTypeValidator(LocType.get, 'Loc');
export const LocAngleValid: ScriptValidator<number, LocAngle> = new ScriptInputLocAngleValidator();
export const LocShapeValid: ScriptValidator<number, LocShape> = new ScriptInputLocShapeValidator();
export const DurationValid: ScriptValidator<number, number> = new ScriptInputDurationValidator();
export const CoordValid: ScriptValidator<number, Position> = new ScriptInputCoordValidator();
export const ParamTypeValid: ScriptValidator<number, ParamType> = new ScriptInputConfigTypeValidator(ParamType.get, 'Param');
export const NpcTypeValid: ScriptValidator<number, NpcType> = new ScriptInputConfigTypeValidator(NpcType.get, 'Npc');
export const NpcStatValid: ScriptValidator<number, number> = new ScriptInputNpcStatValidator();
export const QueueValid: ScriptValidator<number, number> = new ScriptInputQueueValidator();
export const HuntTypeValid: ScriptValidator<number, HuntType> = new ScriptInputConfigTypeValidator(HuntType.get, 'Hunt');
export const NpcModeValid: ScriptValidator<number, number> = new ScriptInputNpcModeValidator();
export const HitTypeValid: ScriptValidator<number, number> = new ScriptInputHitTypeValidator();
export const SpotAnimTypeValid: ScriptValidator<number, SpotanimType> = new ScriptInputConfigTypeValidator(SpotanimType.get, 'Spotanim');
export const EnumTypeValid: ScriptValidator<number, EnumType> = new ScriptInputConfigTypeValidator(EnumType.get, 'Enum');
export const ObjTypeValid: ScriptValidator<number, ObjType> = new ScriptInputConfigTypeValidator(ObjType.get, 'Obj');
export const ObjStackValid: ScriptValidator<number, number> = new ScriptInputObjCountValidator();
export const ObjNotDummyValid: ScriptValidator<number, number> = new ScriptInputObjNotDummyValidator();
export const InvTypeValid: ScriptValidator<number, InvType> = new ScriptInputConfigTypeValidator(InvType.get, 'Inv');
export const CategoryTypeValid: ScriptValidator<number, CategoryType> = new ScriptInputConfigTypeValidator(CategoryType.get, 'Cat');
export const IDKTypeValid: ScriptValidator<number, IdkType> = new ScriptInputConfigTypeValidator(IdkType.get, 'Idk');
export const HuntVisValid: ScriptValidator<number, HuntVis> = new ScriptInputHuntVisValidator();
export const SeqTypeValid: ScriptValidator<number, SeqType> = new ScriptInputConfigTypeValidator(SeqType.get, 'Seq');
export const VarPlayerValid: ScriptValidator<number, VarPlayerType> = new ScriptInputConfigTypeValidator(VarPlayerType.get, 'Varp');
export const VarNpcValid: ScriptValidator<number, VarNpcType> = new ScriptInputConfigTypeValidator(VarNpcType.get, 'Varn');
export const VarSharedValid: ScriptValidator<number, VarSharedType> = new ScriptInputConfigTypeValidator(VarSharedType.get, 'Vars');
export const FontTypeValid: ScriptValidator<number, FontType> = new ScriptInputFontTypeValidator();
export const MesanimValid: ScriptValidator<number, MesanimType> = new ScriptInputConfigTypeValidator(MesanimType.get, 'Mesanim');
export const StructTypeValid: ScriptValidator<number, StructType> = new ScriptInputConfigTypeValidator(StructType.get, 'Struct');
export const DbRowTypeValid: ScriptValidator<number, DbRowType> = new ScriptInputConfigTypeValidator(DbRowType.get, 'Dbrow');
export const DbTableTypeValid: ScriptValidator<number, DbTableType> = new ScriptInputConfigTypeValidator(DbTableType.get, 'Dbtable');

export function check<T, R>(input: T, validator: ScriptValidator<T, R>): R {
    return validator.validate(input);
}

