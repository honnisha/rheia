use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::{block_type_info::BlockTypeInfo, voxel_visibility::VoxelVisibility};


#[derive(Debug, Clone, Copy, Eq, EnumIter, Serialize, Deserialize)]
#[repr(u16)]
pub enum BlockType {
    Air,
    Water,
    Lava,
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    GrassBlock,
    Dirt,
    CoarseDirt,
    Podzol,
    Cobblestone,
    OakPlanks,
    SprucePlanks,
    BirchPlanks,
    JunglePlanks,
    AcaciaPlanks,
    DarkOakPlanks,
    Bedrock,
    Sand,
    RedSand,
    Gravel,
    GoldOre,
    IronOre,
    CoalOre,
    NetherGoldOre,
    OakLog,
    SpruceLog,
    BirchLog,
    JungleLog,
    AcaciaLog,
    DarkOakLog,
    StrippedSpruceLog,
    StrippedBirchLog,
    StrippedJungleLog,
    StrippedAcaciaLog,
    StrippedDarkOakLog,
    StrippedOakLog,
    OakWood,
    SpruceWood,
    BirchWood,
    JungleWood,
    AcaciaWood,
    DarkOakWood,
    StrippedOakWood,
    StrippedSpruceWood,
    StrippedBirchWood,
    StrippedJungleWood,
    StrippedAcaciaWood,
    StrippedDarkOakWood,
    OakLeaves,
    SpruceLeaves,
    BirchLeaves,
    JungleLeaves,
    AcaciaLeaves,
    DarkOakLeaves,
    Sponge,
    WetSponge,
    Glass,
    LapisOre,
    LapisBlock,
    Dispenser,
    Sandstone,
    ChiseledSandstone,
    CutSandstone,
    NoteBlock,
    StickyPiston,
    Piston,
    PistonHead,
    WhiteWool,
    OrangeWool,
    MagentaWool,
    LightBlueWool,
    YellowWool,
    LimeWool,
    PinkWool,
    GrayWool,
    LightGrayWool,
    CyanWool,
    PurpleWool,
    BlueWool,
    BrownWool,
    GreenWool,
    RedWool,
    BlackWool,
    GoldBlock,
    IronBlock,
    Bricks,
    Tnt,
    Bookshelf,
    MossyCobblestone,
    Obsidian,
    Spawner,
    OakStairs,
    Chest,
    DiamondOre,
    DiamondBlock,
    CraftingTable,
    Farmland,
    Furnace,
    OakDoor,
    Ladder,
    CobblestoneStairs,
    IronDoor,
    RedstoneOre,
    Snow,
    Ice,
    SnowBlock,
    Cactus,
    Clay,
    Jukebox,
    OakFence,
    Pumpkin,
    Netherrack,
    SoulSand,
    SoulSoil,
    Basalt,
    PolishedBasalt,
    Glowstone,
    CarvedPumpkin,
    JackOLantern,
    Cake,
    Repeater,
    WhiteStainedGlass,
    OrangeStainedGlass,
    MagentaStainedGlass,
    LightBlueStainedGlass,
    YellowStainedGlass,
    LimeStainedGlass,
    PinkStainedGlass,
    GrayStainedGlass,
    LightGrayStainedGlass,
    CyanStainedGlass,
    PurpleStainedGlass,
    BlueStainedGlass,
    BrownStainedGlass,
    GreenStainedGlass,
    RedStainedGlass,
    BlackStainedGlass,
    OakTrapdoor,
    SpruceTrapdoor,
    BirchTrapdoor,
    JungleTrapdoor,
    AcaciaTrapdoor,
    DarkOakTrapdoor,
    StoneBricks,
    MossyStoneBricks,
    CrackedStoneBricks,
    ChiseledStoneBricks,
    InfestedStone,
    InfestedCobblestone,
    InfestedStoneBricks,
    InfestedMossyStoneBricks,
    InfestedCrackedStoneBricks,
    InfestedChiseledStoneBricks,
    BrownMushroomBlock,
    RedMushroomBlock,
    MushroomStem,
    IronBars,
    Chain,
    GlassPane,
    Melon,
    OakFenceGate,
    BrickStairs,
    StoneBrickStairs,
    Mycelium,
    LilyPad,
    NetherBricks,
    NetherBrickFence,
    NetherBrickStairs,
    EnchantingTable,
    BrewingStand,
    Cauldron,
    EndPortalFrame,
    EndStone,
    DragonEgg,
    RedstoneLamp,
    Cocoa,
    SandstoneStairs,
    EmeraldOre,
    EnderChest,
    EmeraldBlock,
    SpruceStairs,
    BirchStairs,
    JungleStairs,
    CommandBlock,
    Beacon,
    CobblestoneWall,
    MossyCobblestoneWall,
    FlowerPot,
    PottedOakSapling,
    PottedSpruceSapling,
    PottedBirchSapling,
    PottedJungleSapling,
    PottedAcaciaSapling,
    PottedDarkOakSapling,
    PottedFern,
    PottedDandelion,
    PottedPoppy,
    PottedBlueOrchid,
    PottedAllium,
    PottedAzureBluet,
    PottedRedTulip,
    PottedOrangeTulip,
    PottedWhiteTulip,
    PottedPinkTulip,
    PottedOxeyeDaisy,
    PottedCornflower,
    PottedLilyOfTheValley,
    PottedWitherRose,
    PottedRedMushroom,
    PottedBrownMushroom,
    PottedDeadBush,
    PottedCactus,
    SkeletonSkull,
    SkeletonWallSkull,
    WitherSkeletonSkull,
    WitherSkeletonWallSkull,
    ZombieHead,
    ZombieWallHead,
    PlayerHead,
    PlayerWallHead,
    CreeperHead,
    CreeperWallHead,
    DragonHead,
    DragonWallHead,
    Anvil,
    ChippedAnvil,
    DamagedAnvil,
    TrappedChest,
    Comparator,
    DaylightDetector,
    RedstoneBlock,
    NetherQuartzOre,
    Hopper,
    QuartzBlock,
    ChiseledQuartzBlock,
    QuartzPillar,
    QuartzStairs,
    Dropper,
    WhiteTerracotta,
    OrangeTerracotta,
    MagentaTerracotta,
    LightBlueTerracotta,
    YellowTerracotta,
    LimeTerracotta,
    PinkTerracotta,
    GrayTerracotta,
    LightGrayTerracotta,
    CyanTerracotta,
    PurpleTerracotta,
    BlueTerracotta,
    BrownTerracotta,
    GreenTerracotta,
    RedTerracotta,
    BlackTerracotta,
    WhiteStainedGlassPane,
    OrangeStainedGlassPane,
    MagentaStainedGlassPane,
    LightBlueStainedGlassPane,
    YellowStainedGlassPane,
    LimeStainedGlassPane,
    PinkStainedGlassPane,
    GrayStainedGlassPane,
    LightGrayStainedGlassPane,
    CyanStainedGlassPane,
    PurpleStainedGlassPane,
    BlueStainedGlassPane,
    BrownStainedGlassPane,
    GreenStainedGlassPane,
    RedStainedGlassPane,
    BlackStainedGlassPane,
    AcaciaStairs,
    DarkOakStairs,
    SlimeBlock,
    //Barrier,
    IronTrapdoor,
    Prismarine,
    PrismarineBricks,
    DarkPrismarine,
    PrismarineStairs,
    PrismarineBrickStairs,
    DarkPrismarineStairs,
    PrismarineSlab,
    PrismarineBrickSlab,
    DarkPrismarineSlab,
    SeaLantern,
    HayBlock,
    WhiteCarpet,
    OrangeCarpet,
    MagentaCarpet,
    LightBlueCarpet,
    YellowCarpet,
    LimeCarpet,
    PinkCarpet,
    GrayCarpet,
    LightGrayCarpet,
    CyanCarpet,
    PurpleCarpet,
    BlueCarpet,
    BrownCarpet,
    GreenCarpet,
    RedCarpet,
    BlackCarpet,
    Terracotta,
    CoalBlock,
    PackedIce,
    RedSandstone,
    ChiseledRedSandstone,
    CutRedSandstone,
    RedSandstoneStairs,
    OakSlab,
    SpruceSlab,
    BirchSlab,
    JungleSlab,
    AcaciaSlab,
    DarkOakSlab,
    StoneSlab,
    SmoothStoneSlab,
    SandstoneSlab,
    CutSandstoneSlab,
    PetrifiedOakSlab,
    CobblestoneSlab,
    BrickSlab,
    StoneBrickSlab,
    NetherBrickSlab,
    QuartzSlab,
    RedSandstoneSlab,
    CutRedSandstoneSlab,
    PurpurSlab,
    SmoothStone,
    SmoothSandstone,
    SmoothQuartz,
    SmoothRedSandstone,
    SpruceFenceGate,
    BirchFenceGate,
    JungleFenceGate,
    AcaciaFenceGate,
    DarkOakFenceGate,
    SpruceFence,
    BirchFence,
    JungleFence,
    AcaciaFence,
    DarkOakFence,
    SpruceDoor,
    BirchDoor,
    JungleDoor,
    AcaciaDoor,
    DarkOakDoor,
    EndRod,
    ChorusPlant,
    ChorusFlower,
    PurpurBlock,
    PurpurPillar,
    PurpurStairs,
    EndStoneBricks,
    GrassPath,
    RepeatingCommandBlock,
    ChainCommandBlock,
    FrostedIce,
    MagmaBlock,
    NetherWartBlock,
    RedNetherBricks,
    BoneBlock,
    Observer,
    ShulkerBox,
    WhiteShulkerBox,
    OrangeShulkerBox,
    MagentaShulkerBox,
    LightBlueShulkerBox,
    YellowShulkerBox,
    LimeShulkerBox,
    PinkShulkerBox,
    GrayShulkerBox,
    LightGrayShulkerBox,
    CyanShulkerBox,
    PurpleShulkerBox,
    BlueShulkerBox,
    BrownShulkerBox,
    GreenShulkerBox,
    RedShulkerBox,
    BlackShulkerBox,
    WhiteGlazedTerracotta,
    OrangeGlazedTerracotta,
    MagentaGlazedTerracotta,
    LightBlueGlazedTerracotta,
    YellowGlazedTerracotta,
    LimeGlazedTerracotta,
    PinkGlazedTerracotta,
    GrayGlazedTerracotta,
    LightGrayGlazedTerracotta,
    CyanGlazedTerracotta,
    PurpleGlazedTerracotta,
    BlueGlazedTerracotta,
    BrownGlazedTerracotta,
    GreenGlazedTerracotta,
    RedGlazedTerracotta,
    BlackGlazedTerracotta,
    WhiteConcrete,
    OrangeConcrete,
    MagentaConcrete,
    LightBlueConcrete,
    YellowConcrete,
    LimeConcrete,
    PinkConcrete,
    GrayConcrete,
    LightGrayConcrete,
    CyanConcrete,
    PurpleConcrete,
    BlueConcrete,
    BrownConcrete,
    GreenConcrete,
    RedConcrete,
    BlackConcrete,
    WhiteConcretePowder,
    OrangeConcretePowder,
    MagentaConcretePowder,
    LightBlueConcretePowder,
    YellowConcretePowder,
    LimeConcretePowder,
    PinkConcretePowder,
    GrayConcretePowder,
    LightGrayConcretePowder,
    CyanConcretePowder,
    PurpleConcretePowder,
    BlueConcretePowder,
    BrownConcretePowder,
    GreenConcretePowder,
    RedConcretePowder,
    BlackConcretePowder,
    DriedKelpBlock,
    TurtleEgg,
    DeadTubeCoralBlock,
    DeadBrainCoralBlock,
    DeadBubbleCoralBlock,
    DeadFireCoralBlock,
    DeadHornCoralBlock,
    TubeCoralBlock,
    BrainCoralBlock,
    BubbleCoralBlock,
    FireCoralBlock,
    HornCoralBlock,
    SeaPickle,
    BlueIce,
    Conduit,
    Bamboo,
    PottedBamboo,
    PolishedGraniteStairs,
    SmoothRedSandstoneStairs,
    MossyStoneBrickStairs,
    PolishedDioriteStairs,
    MossyCobblestoneStairs,
    EndStoneBrickStairs,
    StoneStairs,
    SmoothSandstoneStairs,
    SmoothQuartzStairs,
    GraniteStairs,
    AndesiteStairs,
    RedNetherBrickStairs,
    PolishedAndesiteStairs,
    DioriteStairs,
    PolishedGraniteSlab,
    SmoothRedSandstoneSlab,
    MossyStoneBrickSlab,
    PolishedDioriteSlab,
    MossyCobblestoneSlab,
    EndStoneBrickSlab,
    SmoothSandstoneSlab,
    SmoothQuartzSlab,
    GraniteSlab,
    AndesiteSlab,
    RedNetherBrickSlab,
    PolishedAndesiteSlab,
    DioriteSlab,
    BrickWall,
    PrismarineWall,
    RedSandstoneWall,
    MossyStoneBrickWall,
    GraniteWall,
    StoneBrickWall,
    NetherBrickWall,
    AndesiteWall,
    RedNetherBrickWall,
    SandstoneWall,
    EndStoneBrickWall,
    DioriteWall,
    Scaffolding,
    Loom,
    Barrel,
    Smoker,
    BlastFurnace,
    CartographyTable,
    FletchingTable,
    Grindstone,
    Lectern,
    SmithingTable,
    Stonecutter,
    Bell,
    Lantern,
    SoulLantern,
    Campfire,
    SoulCampfire,
    WarpedStem,
    StrippedWarpedStem,
    WarpedHyphae,
    StrippedWarpedHyphae,
    WarpedNylium,
    WarpedWartBlock,
    CrimsonStem,
    StrippedCrimsonStem,
    CrimsonHyphae,
    StrippedCrimsonHyphae,
    CrimsonNylium,
    Shroomlight,
    CrimsonPlanks,
    WarpedPlanks,
    CrimsonSlab,
    WarpedSlab,
    CrimsonFence,
    WarpedFence,
    CrimsonTrapdoor,
    WarpedTrapdoor,
    CrimsonFenceGate,
    WarpedFenceGate,
    CrimsonStairs,
    WarpedStairs,
    CrimsonDoor,
    WarpedDoor,
    StructureBlock,
    Jigsaw,
    Composter,
    Target,
    BeeNest,
    Beehive,
    HoneyBlock,
    HoneycombBlock,
    NetheriteBlock,
    AncientDebris,
    CryingObsidian,
    RespawnAnchor,
    PottedCrimsonFungus,
    PottedWarpedFungus,
    PottedCrimsonRoots,
    PottedWarpedRoots,
    Lodestone,
    Blackstone,
    BlackstoneStairs,
    BlackstoneWall,
    BlackstoneSlab,
    PolishedBlackstone,
    PolishedBlackstoneBricks,
    CrackedPolishedBlackstoneBricks,
    ChiseledPolishedBlackstone,
    PolishedBlackstoneBrickSlab,
    PolishedBlackstoneBrickStairs,
    PolishedBlackstoneBrickWall,
    GildedBlackstone,
    PolishedBlackstoneStairs,
    PolishedBlackstoneSlab,
    PolishedBlackstoneWall,
    ChiseledNetherBricks,
    CrackedNetherBricks,
    QuartzBricks,
}

const BLOCK_AIR: BlockTypeInfo = BlockTypeInfo::new_empty();
const BLOCK_WATER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("water.png");
const BLOCK_LAVA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("lava.png");

const BLOCK_GRASS_BLOCK: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("grass_top.png"),
    side_texture: Some("grass_block_side.png"),
    bottom_texture: Some("dirt.png"),
};

const BLOCK_PODZOL: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("podzol_top.png"),
    side_texture: Some("podzol_side.png"),
    bottom_texture: Some("dirt.png"),
};

const BLOCK_DISPENSER: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("dispenser_front_vertical.png"),
    side_texture: Some("dispenser_front.png"),
    bottom_texture: Some("dispenser_front_vertical.png"),
};

const BLOCK_STICKY_PISTON: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("piston_top_sticky.png"),
    side_texture: Some("piston_side.png"),
    bottom_texture: Some("piston_bottom.png"),
};

const BLOCK_PISTON: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("piston_top.png"),
    side_texture: Some("piston_side.png"),
    bottom_texture: Some("piston_bottom.png"),
};

const BLOCK_PISTON_HEAD: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("piston_top.png"),
    side_texture: Some("piston_side.png"),
    bottom_texture: Some("piston_bottom.png"),
};

const BLOCK_TNT: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("tnt_top.png"),
    side_texture: Some("tnt_side.png"),
    bottom_texture: Some("tnt_bottom.png"),
};

const BLOCK_CRAFTING_TABLE: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("crafting_table_top.png"),
    side_texture: Some("crafting_table_front.png"),
    bottom_texture: Some("crafting_table_top.png"),
};

const BLOCK_FURNACE: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("furnace_top.png"),
    side_texture: Some("furnace_front.png"),
    bottom_texture: Some("furnace_top.png"),
};

const BLOCK_BLAST_FURNACE: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("furnace_top.png"),
    side_texture: Some("furnace_front.png"),
    bottom_texture: Some("furnace_vent.png"),
};

const BLOCK_CAKE: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("cake_top.png"),
    side_texture: Some("cake_side.png"),
    bottom_texture: Some("cake_bottom.png"),
};

const BLOCK_MELON: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("melon_top.png"),
    side_texture: Some("melon_side.png"),
    bottom_texture: Some("melon_top.png"),
};

const BLOCK_MYCELIUM: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("mycelium_top.png"),
    side_texture: Some("mycelium_side.png"),
    bottom_texture: Some("mycelium_top.png"),
};

const BLOCK_ENCHANTING_TABLE: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("enchanting_table_top.png"),
    side_texture: Some("enchanting_table_side.png"),
    bottom_texture: Some("enchanting_table_bottom.png"),
};

const BLOCK_CAULDRON: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("cauldron_top.png"),
    side_texture: Some("cauldron_side.png"),
    bottom_texture: Some("cauldron_bottom.png"),
};

const BLOCK_END_PORTAL_FRAME: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("end_portal_frame_top.png"),
    side_texture: Some("end_portal_frame_side.png"),
    bottom_texture: Some("end_stone_bricks.png"),
};

const BLOCK_CACTUS: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("cactus_top.png"),
    side_texture: Some("cactus_side.png"),
    bottom_texture: Some("cactus_bottom.png"),
};

const BLOCK_HOPPER: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("hopper_bottom.png"),
    side_texture: Some("hopper_side.png"),
    bottom_texture: Some("hopper_bottom.png"),
};

const BLOCK_QUARTZ_BLOCK: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("quartz_block_top.png"),
    side_texture: Some("quartz_block_side.png"),
    bottom_texture: Some("quartz_block_bottom.png"),
};

const BLOCK_DROPPER: BlockTypeInfo = BlockTypeInfo {
    voxel_visibility: VoxelVisibility::Opaque,
    top_texture: Some("dropper_front_vertical.png"),
    side_texture: Some("dropper_front.png"),
    bottom_texture: Some("stone.png"),
};

const BLOCK_COMMAND_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");

const BLOCK_COCOA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cocoa_stage2.png");

const BLOCK_POTTED_FERN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fern.png");
const BLOCK_POTTED_DANDELION: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dandelion.png");
const BLOCK_POTTED_POPPY: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("poppy.png");
const BLOCK_POTTED_BLUE_ORCHID: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_orchid.png");
const BLOCK_POTTED_ALLIUM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("allium.png");
const BLOCK_POTTED_AZURE_BLUET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("azure_bluet.png");
const BLOCK_POTTED_RED_TULIP: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_tulip.png");
const BLOCK_POTTED_ORANGE_TULIP: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("orange_tulip.png");
const BLOCK_POTTED_WHITE_TULIP: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("white_tulip.png");
const BLOCK_POTTED_PINK_TULIP: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pink_tulip.png");
const BLOCK_POTTED_OXEYE_DAISY: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oxeye_daisy.png");
const BLOCK_POTTED_CORNFLOWER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cornflower.png");
const BLOCK_POTTED_LILY_OF_THE_VALLEY: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lily_of_the_valley.png");
const BLOCK_POTTED_WITHER_ROSE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("wither_rose.png");
const BLOCK_POTTED_RED_MUSHROOM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_mushroom.png");
const BLOCK_POTTED_BROWN_MUSHROOM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_mushroom.png");
const BLOCK_POTTED_DEAD_BUSH: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dead_bush.png");
const BLOCK_POTTED_CACTUS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cactus_side.png");
const BLOCK_POTTED_BAMBOO: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bamboo.png");
const BLOCK_POTTED_CRIMSON_FUNGUS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_fungus.png");
const BLOCK_POTTED_WARPED_FUNGUS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_fungus.png");
const BLOCK_POTTED_CRIMSON_ROOTS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_roots.png");
const BLOCK_POTTED_WARPED_ROOTS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_roots.png");

const BLOCK_POTTED_OAK_SAPLING: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_sapling.png");
const BLOCK_POTTED_SPRUCE_SAPLING: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_sapling.png");
const BLOCK_POTTED_BIRCH_SAPLING: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_sapling.png");
const BLOCK_POTTED_JUNGLE_SAPLING: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_sapling.png");
const BLOCK_POTTED_ACACIA_SAPLING: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_sapling.png");
const BLOCK_POTTED_DARK_OAK_SAPLING: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_sapling.png");

const BLOCK_OAK_FENCE_GATE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fence_oak.png");
const BLOCK_SPRUCE_FENCE_GATE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fence_spruce.png");
const BLOCK_BIRCH_FENCE_GATE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fence_birch.png");
const BLOCK_JUNGLE_FENCE_GATE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fence_jungle.png");
const BLOCK_ACACIA_FENCE_GATE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fence_acacia.png");
const BLOCK_DARK_OAK_FENCE_GATE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_fence_oak.png");
const BLOCK_CRIMSON_FENCE_GATE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fence_crimson.png");
const BLOCK_WARPED_FENCE_GATE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fence_warped.png");

const BLOCK_OAK_FENCE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_planks.png");
const BLOCK_NETHER_BRICK_FENCE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("nether_bricks.png");
const BLOCK_SPRUCE_FENCE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_planks.png");
const BLOCK_BIRCH_FENCE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_planks.png");
const BLOCK_JUNGLE_FENCE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_planks.png");
const BLOCK_ACACIA_FENCE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_planks.png");
const BLOCK_DARK_OAK_FENCE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_planks.png");
const BLOCK_CRIMSON_FENCE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_planks.png");
const BLOCK_WARPED_FENCE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_planks.png");

const BLOCK_PRISMARINE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("prismarine_brick_double_slab.png");
const BLOCK_PRISMARINE_BRICK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("prismarine_brick_double_slab.png");
const BLOCK_DARK_PRISMARINE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_prismarine_planks.png");
const BLOCK_OAK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_planks.png");
const BLOCK_SPRUCE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_planks.png");
const BLOCK_BIRCH_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_planks.png");
const BLOCK_JUNGLE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_planks.png");
const BLOCK_ACACIA_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_planks.png");
const BLOCK_DARK_OAK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_planks.png");
const BLOCK_STONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone.png");
const BLOCK_SMOOTH_STONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone.png");
const BLOCK_SANDSTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("sandstone.png");
const BLOCK_CUT_SANDSTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("sandstone.png");
const BLOCK_PETRIFIED_OAK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_planks.png");
const BLOCK_COBBLESTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cobblestone.png");
const BLOCK_BRICK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brick_top.png");
const BLOCK_STONE_BRICK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone.png");
const BLOCK_NETHER_BRICK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("nether_brick_slab.png");
const BLOCK_QUARTZ_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("quartz_slab.png");
const BLOCK_RED_SANDSTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_sandstone.png");
const BLOCK_CUT_RED_SANDSTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cut_red_sandstone.png");
const BLOCK_PURPUR_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purpur_block.png");

const BLOCK_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("beacon_glass.png");
const BLOCK_WHITE_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("white_stained_glass.png");
const BLOCK_ORANGE_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("orange_stained_glass.png");
const BLOCK_MAGENTA_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("magenta_stained_glass.png");
const BLOCK_LIGHT_BLUE_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("light_blue_stained_glass.png");
const BLOCK_YELLOW_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("yellow_stained_glass.png");
const BLOCK_LIME_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("lime_stained_glass.png");
const BLOCK_PINK_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("pink_stained_glass.png");
const BLOCK_GRAY_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("gray_stained_glass.png");
const BLOCK_LIGHT_GRAY_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("light_gray_stained_glass.png");
const BLOCK_CYAN_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("cyan_stained_glass.png");
const BLOCK_PURPLE_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("purple_stained_glass.png");
const BLOCK_BLUE_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("blue_stained_glass.png");
const BLOCK_BROWN_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("brown_stained_glass.png");
const BLOCK_GREEN_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("green_stained_glass.png");
const BLOCK_RED_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("red_stained_glass.png");
const BLOCK_BLACK_STAINED_GLASS_PANE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_translucent("black_stained_glass.png");

const BLOCK_OAK_DOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_door_top.png");
const BLOCK_IRON_DOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("iron_door_top.png");
const BLOCK_SPRUCE_DOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_door_top.png");
const BLOCK_BIRCH_DOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_door_top.png");
const BLOCK_JUNGLE_DOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_door_top.png");
const BLOCK_ACACIA_DOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_door_top.png");
const BLOCK_DARK_OAK_DOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_door_top.png");
const BLOCK_CRIMSON_DOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_door_top.png");
const BLOCK_WARPED_DOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_door_top.png");

const BLOCK_OAK_TRAPDOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_trapdoor.png");
const BLOCK_SPRUCE_TRAPDOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_trapdoor.png");
const BLOCK_BIRCH_TRAPDOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_trapdoor.png");
const BLOCK_JUNGLE_TRAPDOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_trapdoor.png");
const BLOCK_ACACIA_TRAPDOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_trapdoor.png");
const BLOCK_DARK_OAK_TRAPDOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_trapdoor.png");
const BLOCK_IRON_TRAPDOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("iron_trapdoor.png");
const BLOCK_CRIMSON_TRAPDOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_trapdoor.png");
const BLOCK_WARPED_TRAPDOOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_trapdoor.png");

const BLOCK_STONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone.png");
const BLOCK_GRANITE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("granite.png");
const BLOCK_POLISHED_GRANITE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_granite.png");
const BLOCK_DIORITE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("diorite.png");
const BLOCK_POLISHED_DIORITE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_diorite.png");
const BLOCK_ANDESITE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("andesite.png");
const BLOCK_POLISHED_ANDESITE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_andesite.png");
const BLOCK_DIRT: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dirt.png");
const BLOCK_COARSE_DIRT: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("coarse_dirt.png");
const BLOCK_COBBLESTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cobblestone.png");
const BLOCK_OAK_PLANKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_planks.png");
const BLOCK_SPRUCE_PLANKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_planks.png");
const BLOCK_BIRCH_PLANKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_planks.png");
const BLOCK_JUNGLE_PLANKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_planks.png");
const BLOCK_ACACIA_PLANKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_planks.png");
const BLOCK_DARK_OAK_PLANKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_planks.png");
const BLOCK_BEDROCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_SAND: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("sand.png");
const BLOCK_RED_SAND: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_sand.png");
const BLOCK_GRAVEL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gravel.png");
const BLOCK_GOLD_ORE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gold_ore.png");
const BLOCK_IRON_ORE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("iron_ore.png");
const BLOCK_COAL_ORE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("coal_ore.png");
const BLOCK_NETHER_GOLD_ORE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("nether_gold_ore.png");
const BLOCK_OAK_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_log.png");
const BLOCK_SPRUCE_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_log.png");
const BLOCK_BIRCH_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_log.png");
const BLOCK_JUNGLE_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_log.png");
const BLOCK_ACACIA_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_log.png");
const BLOCK_DARK_OAK_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_log.png");
const BLOCK_STRIPPED_SPRUCE_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_spruce_log.png");
const BLOCK_STRIPPED_BIRCH_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_birch_log.png");
const BLOCK_STRIPPED_JUNGLE_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_jungle_log.png");
const BLOCK_STRIPPED_ACACIA_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_acacia_log.png");
const BLOCK_STRIPPED_DARK_OAK_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_dark_oak_log.png");
const BLOCK_STRIPPED_OAK_LOG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_oak_log.png");

const BLOCK_SKELETON_SKULL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_SKELETON_WALL_SKULL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_WITHER_SKELETON_SKULL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_WITHER_SKELETON_WALL_SKULL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");

const BLOCK_ZOMBIE_HEAD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_ZOMBIE_WALL_HEAD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_PLAYER_HEAD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_PLAYER_WALL_HEAD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_CREEPER_HEAD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_CREEPER_WALL_HEAD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_DRAGON_HEAD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_DRAGON_WALL_HEAD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");

const BLOCK_ANVIL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("anvil_top.png");
const BLOCK_CHIPPED_ANVIL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("anvil_top.png");
const BLOCK_DAMAGED_ANVIL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("anvil_top.png");

const BLOCK_OAK_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_log.png");
const BLOCK_SPRUCE_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_log.png");
const BLOCK_BIRCH_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_log.png");
const BLOCK_JUNGLE_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_log.png");
const BLOCK_ACACIA_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_log.png");
const BLOCK_DARK_OAK_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_log.png");
const BLOCK_STRIPPED_OAK_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_oak_log.png");
const BLOCK_STRIPPED_SPRUCE_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_spruce_log.png");
const BLOCK_STRIPPED_BIRCH_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_birch_log.png");
const BLOCK_STRIPPED_JUNGLE_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_jungle_log.png");
const BLOCK_STRIPPED_ACACIA_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_acacia_log.png");
const BLOCK_STRIPPED_DARK_OAK_WOOD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_dark_oak_log.png");

const BLOCK_CHEST: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_ENDER_CHEST: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");
const BLOCK_TRAPPED_CHEST: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");

const BLOCK_DAYLIGHT_DETECTOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bedrock.png");

const BLOCK_OAK_LEAVES: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_leaves.png");
const BLOCK_SPRUCE_LEAVES: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_leaves.png");
const BLOCK_BIRCH_LEAVES: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_leaves.png");
const BLOCK_JUNGLE_LEAVES: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_leaves.png");
const BLOCK_ACACIA_LEAVES: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_leaves.png");
const BLOCK_DARK_OAK_LEAVES: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_leaves.png");
const BLOCK_SPONGE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("sponge.png");
const BLOCK_WET_SPONGE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("wet_sponge.png");
const BLOCK_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("glass.png");
const BLOCK_LAPIS_ORE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lapis_ore.png");
const BLOCK_LAPIS_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lapis_block.png");

const BLOCK_SANDSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("sandstone.png");
const BLOCK_CHISELED_SANDSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chiseled_sandstone.png");
const BLOCK_CUT_SANDSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cut_sandstone.png");
const BLOCK_NOTE_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("note_block.png");

const BLOCK_WHITE_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("white_wool.png");
const BLOCK_ORANGE_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("orange_wool.png");
const BLOCK_MAGENTA_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("magenta_wool.png");
const BLOCK_LIGHT_BLUE_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_blue_wool.png");
const BLOCK_YELLOW_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("yellow_wool.png");
const BLOCK_LIME_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lime_wool.png");
const BLOCK_PINK_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pink_wool.png");
const BLOCK_GRAY_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gray_wool.png");
const BLOCK_LIGHT_GRAY_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_gray_wool.png");
const BLOCK_CYAN_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cyan_wool.png");
const BLOCK_PURPLE_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purple_wool.png");
const BLOCK_BLUE_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_wool.png");
const BLOCK_BROWN_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_wool.png");
const BLOCK_GREEN_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("green_wool.png");
const BLOCK_RED_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_wool.png");
const BLOCK_BLACK_WOOL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("black_wool.png");
const BLOCK_GOLD_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gold_block.png");
const BLOCK_IRON_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("iron_block.png");
const BLOCK_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bricks.png");
const BLOCK_BOOKSHELF: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bookshelf.png");
const BLOCK_MOSSY_COBBLESTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mossy_cobblestone.png");
const BLOCK_OBSIDIAN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("obsidian.png");
const BLOCK_SPAWNER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spawner.png");
const BLOCK_OAK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("oak_planks.png");
const BLOCK_DIAMOND_ORE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("diamond_ore.png");
const BLOCK_DIAMOND_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("diamond_block.png");
const BLOCK_FARMLAND: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("farmland.png");
const BLOCK_LADDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("ladder.png");
const BLOCK_COBBLESTONE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cobblestone_planks.png");
const BLOCK_REDSTONE_ORE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("redstone_ore.png");
const BLOCK_SNOW: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("snow.png");
const BLOCK_ICE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("ice.png");
const BLOCK_SNOW_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("snow_block.png");
const BLOCK_CLAY: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("clay.png");
const BLOCK_JUKEBOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jukebox.png");
const BLOCK_PUMPKIN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pumpkin.png");
const BLOCK_NETHERRACK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("netherrack.png");
const BLOCK_SOUL_SAND: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("soul_sand.png");
const BLOCK_SOUL_SOIL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("soul_soil.png");
const BLOCK_BASALT: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("basalt.png");
const BLOCK_POLISHED_BASALT: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_basalt.png");
const BLOCK_GLOWSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("glowstone.png");
const BLOCK_CARVED_PUMPKIN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("carved_pumpkin.png");
const BLOCK_JACK_O_LANTERN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jack_o_lantern.png");
const BLOCK_REPEATER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("repeater.png");
const BLOCK_WHITE_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("white_stained_glass.png");
const BLOCK_ORANGE_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("orange_stained_glass.png");
const BLOCK_MAGENTA_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("magenta_stained_glass.png");
const BLOCK_LIGHT_BLUE_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_blue_stained_glass.png");
const BLOCK_YELLOW_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("yellow_stained_glass.png");
const BLOCK_LIME_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lime_stained_glass.png");
const BLOCK_PINK_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pink_stained_glass.png");
const BLOCK_GRAY_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gray_stained_glass.png");
const BLOCK_LIGHT_GRAY_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_gray_stained_glass.png");
const BLOCK_CYAN_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cyan_stained_glass.png");
const BLOCK_PURPLE_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purple_stained_glass.png");
const BLOCK_BLUE_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_stained_glass.png");
const BLOCK_BROWN_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_stained_glass.png");
const BLOCK_GREEN_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("green_stained_glass.png");
const BLOCK_RED_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_stained_glass.png");
const BLOCK_BLACK_STAINED_GLASS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("black_stained_glass.png");
const BLOCK_STONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone_bricks.png");
const BLOCK_MOSSY_STONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mossy_stone_bricks.png");
const BLOCK_CRACKED_STONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cracked_stone_bricks.png");
const BLOCK_CHISELED_STONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chiseled_stone_bricks.png");
const BLOCK_INFESTED_STONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone.png");
const BLOCK_INFESTED_COBBLESTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cobblestone.png");
const BLOCK_INFESTED_STONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone_bricks.png");
const BLOCK_INFESTED_MOSSY_STONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mossy_stone_bricks.png");
const BLOCK_INFESTED_CRACKED_STONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cracked_stone_bricks.png");
const BLOCK_INFESTED_CHISELED_STONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chiseled_stone_bricks.png");
const BLOCK_BROWN_MUSHROOM_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_mushroom_block.png");
const BLOCK_RED_MUSHROOM_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_mushroom_block.png");
const BLOCK_MUSHROOM_STEM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mushroom_stem.png");
const BLOCK_IRON_BARS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("iron_bars.png");
const BLOCK_CHAIN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chain.png");
const BLOCK_BRICK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brick_stairs.png");
const BLOCK_STONE_BRICK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone.png");
const BLOCK_LILY_PAD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lily_pad.png");
const BLOCK_NETHER_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("nether_bricks.png");
const BLOCK_NETHER_BRICK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("nether_bricks.png");
const BLOCK_BREWING_STAND: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brewing_stand.png");
const BLOCK_END_STONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("end_stone.png");
const BLOCK_DRAGON_EGG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dragon_egg.png");
const BLOCK_REDSTONE_LAMP: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("redstone_lamp.png");
const BLOCK_SANDSTONE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("sandstone.png");
const BLOCK_EMERALD_ORE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("emerald_ore.png");
const BLOCK_EMERALD_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("emerald_block.png");
const BLOCK_SPRUCE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("spruce_planks.png");
const BLOCK_BIRCH_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("birch_planks.png");
const BLOCK_JUNGLE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jungle_planks.png");
const BLOCK_BEACON: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("beacon.png");
const BLOCK_COBBLESTONE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cobblestone.png");
const BLOCK_MOSSY_COBBLESTONE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mossy_cobblestone.png");
const BLOCK_FLOWER_POT: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("flower_pot.png");
const BLOCK_COMPARATOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("comparator.png");
const BLOCK_REDSTONE_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("redstone_block.png");
const BLOCK_NETHER_QUARTZ_ORE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("nether_quartz_ore.png");
const BLOCK_CHISELED_QUARTZ_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chiseled_quartz_block.png");
const BLOCK_QUARTZ_PILLAR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("quartz_pillar.png");
const BLOCK_QUARTZ_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("quartz_bricks.png");
const BLOCK_WHITE_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("white_terracotta.png");
const BLOCK_ORANGE_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("orange_terracotta.png");
const BLOCK_MAGENTA_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("magenta_terracotta.png");
const BLOCK_LIGHT_BLUE_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_blue_terracotta.png");
const BLOCK_YELLOW_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("yellow_terracotta.png");
const BLOCK_LIME_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lime_terracotta.png");
const BLOCK_PINK_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pink_terracotta.png");
const BLOCK_GRAY_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gray_terracotta.png");
const BLOCK_LIGHT_GRAY_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_gray_terracotta.png");
const BLOCK_CYAN_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cyan_terracotta.png");
const BLOCK_PURPLE_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purple_terracotta.png");
const BLOCK_BLUE_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_terracotta.png");
const BLOCK_BROWN_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_terracotta.png");
const BLOCK_GREEN_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("green_terracotta.png");
const BLOCK_RED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_terracotta.png");
const BLOCK_BLACK_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("black_terracotta.png");
const BLOCK_ACACIA_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("acacia_planks.png");
const BLOCK_DARK_OAK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_oak_planks.png");
const BLOCK_SLIME_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("slime_block.png");
const BLOCK_PRISMARINE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("prismarine.png");
const BLOCK_PRISMARINE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("prismarine_bricks.png");
const BLOCK_DARK_PRISMARINE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_prismarine.png");
const BLOCK_PRISMARINE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("prismarine_planks.png");
const BLOCK_PRISMARINE_BRICK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("prismarine_brick_planks.png");
const BLOCK_DARK_PRISMARINE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dark_prismarine_planks.png");
const BLOCK_SEA_LANTERN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("sea_lantern.png");
const BLOCK_HAY_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("hay_block.png");
const BLOCK_WHITE_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("white_carpet.png");
const BLOCK_ORANGE_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("orange_carpet.png");
const BLOCK_MAGENTA_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("magenta_carpet.png");
const BLOCK_LIGHT_BLUE_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_blue_carpet.png");
const BLOCK_YELLOW_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("yellow_carpet.png");
const BLOCK_LIME_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lime_carpet.png");
const BLOCK_PINK_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pink_carpet.png");
const BLOCK_GRAY_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gray_carpet.png");
const BLOCK_LIGHT_GRAY_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_gray_carpet.png");
const BLOCK_CYAN_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cyan_carpet.png");
const BLOCK_PURPLE_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purple_carpet.png");
const BLOCK_BLUE_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_carpet.png");
const BLOCK_BROWN_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_carpet.png");
const BLOCK_GREEN_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("green_carpet.png");
const BLOCK_RED_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_carpet.png");
const BLOCK_BLACK_CARPET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("black_carpet.png");
const BLOCK_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("terracotta.png");
const BLOCK_COAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("coal_block.png");
const BLOCK_PACKED_ICE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("packed_ice.png");
const BLOCK_RED_SANDSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_sandstone.png");
const BLOCK_CHISELED_RED_SANDSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chiseled_red_sandstone.png");
const BLOCK_CUT_RED_SANDSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cut_red_sandstone.png");
const BLOCK_RED_SANDSTONE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_sandstone_planks.png");
const BLOCK_SMOOTH_STONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smooth_stone.png");
const BLOCK_SMOOTH_SANDSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smooth_sandstone.png");
const BLOCK_SMOOTH_QUARTZ: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smooth_quartz.png");
const BLOCK_SMOOTH_RED_SANDSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_sandstone.png");
const BLOCK_END_ROD: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("end_rod.png");
const BLOCK_CHORUS_PLANT: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chorus_plant.png");
const BLOCK_CHORUS_FLOWER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chorus_flower.png");
const BLOCK_PURPUR_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purpur_block.png");
const BLOCK_PURPUR_PILLAR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purpur_pillar.png");
const BLOCK_PURPUR_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purpur_planks.png");
const BLOCK_END_STONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("end_stone_planks.png");
const BLOCK_GRASS_PATH: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("grass_path.png");
const BLOCK_REPEATING_COMMAND_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("repeating_command_block.png");
const BLOCK_CHAIN_COMMAND_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chain_command_block.png");
const BLOCK_FROSTED_ICE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("frosted_ice.png");
const BLOCK_MAGMA_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("magma_block.png");
const BLOCK_NETHER_WART_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("nether_wart_block.png");
const BLOCK_RED_NETHER_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_nether_bricks.png");
const BLOCK_BONE_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bone_block.png");
const BLOCK_OBSERVER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("observer.png");
const BLOCK_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("shulker_box.png");
const BLOCK_WHITE_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("white_shulker_box.png");
const BLOCK_ORANGE_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("orange_shulker_box.png");
const BLOCK_MAGENTA_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("magenta_shulker_box.png");
const BLOCK_LIGHT_BLUE_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_blue_shulker_box.png");
const BLOCK_YELLOW_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("yellow_shulker_box.png");
const BLOCK_LIME_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lime_shulker_box.png");
const BLOCK_PINK_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pink_shulker_box.png");
const BLOCK_GRAY_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gray_shulker_box.png");
const BLOCK_LIGHT_GRAY_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_gray_shulker_box.png");
const BLOCK_CYAN_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cyan_shulker_box.png");
const BLOCK_PURPLE_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purple_shulker_box.png");
const BLOCK_BLUE_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_shulker_box.png");
const BLOCK_BROWN_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_shulker_box.png");
const BLOCK_GREEN_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("green_shulker_box.png");
const BLOCK_RED_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_shulker_box.png");
const BLOCK_BLACK_SHULKER_BOX: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("black_shulker_box.png");
const BLOCK_WHITE_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("white_glazed_terracotta.png");
const BLOCK_ORANGE_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("orange_glazed_terracotta.png");
const BLOCK_MAGENTA_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("magenta_glazed_terracotta.png");
const BLOCK_LIGHT_BLUE_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_blue_glazed_terracotta.png");
const BLOCK_YELLOW_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("yellow_glazed_terracotta.png");
const BLOCK_LIME_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lime_glazed_terracotta.png");
const BLOCK_PINK_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pink_glazed_terracotta.png");
const BLOCK_GRAY_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gray_glazed_terracotta.png");
const BLOCK_LIGHT_GRAY_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_gray_glazed_terracotta.png");
const BLOCK_CYAN_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cyan_glazed_terracotta.png");
const BLOCK_PURPLE_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purple_glazed_terracotta.png");
const BLOCK_BLUE_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_glazed_terracotta.png");
const BLOCK_BROWN_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_glazed_terracotta.png");
const BLOCK_GREEN_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("green_glazed_terracotta.png");
const BLOCK_RED_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_glazed_terracotta.png");
const BLOCK_BLACK_GLAZED_TERRACOTTA: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("black_glazed_terracotta.png");
const BLOCK_WHITE_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("white_concrete.png");
const BLOCK_ORANGE_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("orange_concrete.png");
const BLOCK_MAGENTA_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("magenta_concrete.png");
const BLOCK_LIGHT_BLUE_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_blue_concrete.png");
const BLOCK_YELLOW_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("yellow_concrete.png");
const BLOCK_LIME_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lime_concrete.png");
const BLOCK_PINK_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pink_concrete.png");
const BLOCK_GRAY_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gray_concrete.png");
const BLOCK_LIGHT_GRAY_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_gray_concrete.png");
const BLOCK_CYAN_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cyan_concrete.png");
const BLOCK_PURPLE_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purple_concrete.png");
const BLOCK_BLUE_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_concrete.png");
const BLOCK_BROWN_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_concrete.png");
const BLOCK_GREEN_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("green_concrete.png");
const BLOCK_RED_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_concrete.png");
const BLOCK_BLACK_CONCRETE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("black_concrete.png");
const BLOCK_WHITE_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("white_concrete_powder.png");
const BLOCK_ORANGE_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("orange_concrete_powder.png");
const BLOCK_MAGENTA_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("magenta_concrete_powder.png");
const BLOCK_LIGHT_BLUE_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_blue_concrete_powder.png");
const BLOCK_YELLOW_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("yellow_concrete_powder.png");
const BLOCK_LIME_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lime_concrete_powder.png");
const BLOCK_PINK_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("pink_concrete_powder.png");
const BLOCK_GRAY_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gray_concrete_powder.png");
const BLOCK_LIGHT_GRAY_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("light_gray_concrete_powder.png");
const BLOCK_CYAN_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cyan_concrete_powder.png");
const BLOCK_PURPLE_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("purple_concrete_powder.png");
const BLOCK_BLUE_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_concrete_powder.png");
const BLOCK_BROWN_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brown_concrete_powder.png");
const BLOCK_GREEN_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("green_concrete_powder.png");
const BLOCK_RED_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_concrete_powder.png");
const BLOCK_BLACK_CONCRETE_POWDER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("black_concrete_powder.png");
const BLOCK_DRIED_KELP_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dried_kelp_block.png");
const BLOCK_TURTLE_EGG: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("turtle_egg.png");
const BLOCK_DEAD_TUBE_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dead_tube_coral_block.png");
const BLOCK_DEAD_BRAIN_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dead_brain_coral_block.png");
const BLOCK_DEAD_BUBBLE_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dead_bubble_coral_block.png");
const BLOCK_DEAD_FIRE_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dead_fire_coral_block.png");
const BLOCK_DEAD_HORN_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("dead_horn_coral_block.png");
const BLOCK_TUBE_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("tube_coral_block.png");
const BLOCK_BRAIN_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brain_coral_block.png");
const BLOCK_BUBBLE_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bubble_coral_block.png");
const BLOCK_FIRE_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fire_coral_block.png");
const BLOCK_HORN_CORAL_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("horn_coral_block.png");
const BLOCK_SEA_PICKLE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("sea_pickle.png");
const BLOCK_BLUE_ICE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blue_ice.png");
const BLOCK_CONDUIT: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("conduit.png");
const BLOCK_BAMBOO: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bamboo.png");
const BLOCK_POLISHED_GRANITE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_granite.png");
const BLOCK_SMOOTH_RED_SANDSTONE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smooth_red_sandstone.png");
const BLOCK_MOSSY_STONE_BRICK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mossy_stone_bricks.png");
const BLOCK_POLISHED_DIORITE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_diorite.png");
const BLOCK_MOSSY_COBBLESTONE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mossy_cobblestone.png");
const BLOCK_END_STONE_BRICK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("end_stone_brick.png");
const BLOCK_STONE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone_stairs.png");
const BLOCK_SMOOTH_SANDSTONE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smooth_sandstone.png");
const BLOCK_SMOOTH_QUARTZ_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smooth_quartz.png");
const BLOCK_GRANITE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("granite_stairs.png");
const BLOCK_ANDESITE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("andesite_stairs.png");
const BLOCK_RED_NETHER_BRICK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_nether_brick.png");
const BLOCK_POLISHED_ANDESITE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_andesite.png");
const BLOCK_DIORITE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("diorite.png");
const BLOCK_POLISHED_GRANITE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_granite_slab.png");
const BLOCK_SMOOTH_RED_SANDSTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smooth_red_sandstone_slab.png");
const BLOCK_MOSSY_STONE_BRICK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mossy_stone_brick_slab.png");
const BLOCK_POLISHED_DIORITE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_diorite_slab.png");
const BLOCK_MOSSY_COBBLESTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mossy_cobblestone_slab.png");
const BLOCK_END_STONE_BRICK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("end_stone_brick_slab.png");
const BLOCK_SMOOTH_SANDSTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smooth_sandstone_slab.png");
const BLOCK_SMOOTH_QUARTZ_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smooth_quartz_slab.png");
const BLOCK_GRANITE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("granite_slab.png");
const BLOCK_ANDESITE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("andesite_slab.png");
const BLOCK_RED_NETHER_BRICK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_nether_brick_slab.png");
const BLOCK_POLISHED_ANDESITE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_andesite_slab.png");
const BLOCK_DIORITE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("diorite_slab.png");
const BLOCK_BRICK_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("brick_wall.png");
const BLOCK_PRISMARINE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("prismarine_wall.png");
const BLOCK_RED_SANDSTONE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_sandstone_wall.png");
const BLOCK_MOSSY_STONE_BRICK_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("mossy_stone_brick_wall.png");
const BLOCK_GRANITE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("granite_wall.png");
const BLOCK_STONE_BRICK_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stone_brick_wall.png");
const BLOCK_NETHER_BRICK_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("nether_brick_wall.png");
const BLOCK_ANDESITE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("andesite_wall.png");
const BLOCK_RED_NETHER_BRICK_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("red_nether_brick_wall.png");
const BLOCK_SANDSTONE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("sandstone_wall.png");
const BLOCK_END_STONE_BRICK_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("end_stone_brick_wall.png");
const BLOCK_DIORITE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("diorite_wall.png");
const BLOCK_SCAFFOLDING: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("scaffolding.png");
const BLOCK_LOOM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("loom.png");
const BLOCK_BARREL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("barrel.png");
const BLOCK_SMOKER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smoker.png");
const BLOCK_CARTOGRAPHY_TABLE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cartography_table.png");
const BLOCK_FLETCHING_TABLE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("fletching_table.png");
const BLOCK_GRINDSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("grindstone.png");
const BLOCK_LECTERN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lectern.png");
const BLOCK_SMITHING_TABLE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("smithing_table.png");
const BLOCK_STONECUTTER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stonecutter.png");
const BLOCK_BELL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bell.png");
const BLOCK_LANTERN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lantern.png");
const BLOCK_SOUL_LANTERN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("soul_lantern.png");
const BLOCK_CAMPFIRE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("campfire.png");
const BLOCK_SOUL_CAMPFIRE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("soul_campfire.png");
const BLOCK_WARPED_STEM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_stem.png");
const BLOCK_STRIPPED_WARPED_STEM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_warped_stem.png");
const BLOCK_WARPED_HYPHAE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_hyphae.png");
const BLOCK_STRIPPED_WARPED_HYPHAE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_warped_hyphae.png");
const BLOCK_WARPED_NYLIUM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_nylium.png");
const BLOCK_WARPED_WART_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_wart_block.png");
const BLOCK_CRIMSON_STEM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_stem.png");
const BLOCK_STRIPPED_CRIMSON_STEM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_crimson_stem.png");
const BLOCK_CRIMSON_HYPHAE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_hyphae.png");
const BLOCK_STRIPPED_CRIMSON_HYPHAE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("stripped_crimson_hyphae.png");
const BLOCK_CRIMSON_NYLIUM: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_nylium.png");
const BLOCK_SHROOMLIGHT: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("shroomlight.png");
const BLOCK_CRIMSON_PLANKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_planks.png");
const BLOCK_WARPED_PLANKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_planks.png");
const BLOCK_CRIMSON_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_slab.png");
const BLOCK_WARPED_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_planks.png");
const BLOCK_CRIMSON_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crimson_planks_alt.png");
const BLOCK_WARPED_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("warped_planks.png");
const BLOCK_STRUCTURE_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("structure_block.png");
const BLOCK_JIGSAW: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("jigsaw.png");
const BLOCK_COMPOSTER: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("composter.png");
const BLOCK_TARGET: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("target.png");
const BLOCK_BEE_NEST: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("bee_nest.png");
const BLOCK_BEEHIVE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("beehive.png");
const BLOCK_HONEY_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("honey_block.png");
const BLOCK_HONEYCOMB_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("honeycomb_block.png");
const BLOCK_NETHERITE_BLOCK: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("netherite_block.png");
const BLOCK_ANCIENT_DEBRIS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("ancient_debris.png");
const BLOCK_CRYING_OBSIDIAN: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("crying_obsidian.png");
const BLOCK_RESPAWN_ANCHOR: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("respawn_anchor.png");
const BLOCK_LODESTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("lodestone.png");
const BLOCK_BLACKSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blackstone.png");
const BLOCK_BLACKSTONE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blackstone.png");
const BLOCK_BLACKSTONE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blackstone_wall.png");
const BLOCK_BLACKSTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("blackstone_slab.png");
const BLOCK_POLISHED_BLACKSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_blackstone.png");
const BLOCK_POLISHED_BLACKSTONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_blackstone_bricks.png");
const BLOCK_CRACKED_POLISHED_BLACKSTONE_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cracked_polished_blackstone_bricks.png");
const BLOCK_CHISELED_POLISHED_BLACKSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chiseled_polished_blackstone.png");
const BLOCK_POLISHED_BLACKSTONE_BRICK_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_blackstone_brick.png");
const BLOCK_POLISHED_BLACKSTONE_BRICK_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_blackstone_brick.png");
const BLOCK_POLISHED_BLACKSTONE_BRICK_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_blackstone_brick.png");
const BLOCK_GILDED_BLACKSTONE: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("gilded_blackstone.png");
const BLOCK_POLISHED_BLACKSTONE_STAIRS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_blackstone.png");
const BLOCK_POLISHED_BLACKSTONE_SLAB: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_blackstone.png");
const BLOCK_POLISHED_BLACKSTONE_WALL: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("polished_blackstone.png");
const BLOCK_CHISELED_NETHER_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("chiseled_nether_bricks.png");
const BLOCK_CRACKED_NETHER_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("cracked_nether_bricks.png");
const BLOCK_QUARTZ_BRICKS: BlockTypeInfo = BlockTypeInfo::new_opaque_mono_side("quartz_bricks.png");

pub fn get_block_type_info(block_type: &BlockType) -> Option<&'static BlockTypeInfo> {
    match block_type {
        BlockType::Air => Some(&BLOCK_AIR),
        BlockType::Water => Some(&BLOCK_WATER),
        BlockType::Lava => Some(&BLOCK_LAVA),
        BlockType::Stone => Some(&BLOCK_STONE),
        BlockType::Granite => Some(&BLOCK_GRANITE),
        BlockType::PolishedGranite => Some(&BLOCK_POLISHED_GRANITE),
        BlockType::Diorite => Some(&BLOCK_DIORITE),
        BlockType::PolishedDiorite => Some(&BLOCK_POLISHED_DIORITE),
        BlockType::Andesite => Some(&BLOCK_ANDESITE),
        BlockType::PolishedAndesite => Some(&BLOCK_POLISHED_ANDESITE),
        BlockType::GrassBlock => Some(&BLOCK_GRASS_BLOCK),
        BlockType::Dirt => Some(&BLOCK_DIRT),
        BlockType::CoarseDirt => Some(&BLOCK_COARSE_DIRT),
        BlockType::Podzol => Some(&BLOCK_PODZOL),
        BlockType::Cobblestone => Some(&BLOCK_COBBLESTONE),
        BlockType::OakPlanks => Some(&BLOCK_OAK_PLANKS),
        BlockType::SprucePlanks => Some(&BLOCK_SPRUCE_PLANKS),
        BlockType::BirchPlanks => Some(&BLOCK_BIRCH_PLANKS),
        BlockType::JunglePlanks => Some(&BLOCK_JUNGLE_PLANKS),
        BlockType::AcaciaPlanks => Some(&BLOCK_ACACIA_PLANKS),
        BlockType::DarkOakPlanks => Some(&BLOCK_DARK_OAK_PLANKS),
        BlockType::Bedrock => Some(&BLOCK_BEDROCK),
        BlockType::Sand => Some(&BLOCK_SAND),
        BlockType::RedSand => Some(&BLOCK_RED_SAND),
        BlockType::Gravel => Some(&BLOCK_GRAVEL),
        BlockType::GoldOre => Some(&BLOCK_GOLD_ORE),
        BlockType::IronOre => Some(&BLOCK_IRON_ORE),
        BlockType::CoalOre => Some(&BLOCK_COAL_ORE),
        BlockType::NetherGoldOre => Some(&BLOCK_NETHER_GOLD_ORE),
        BlockType::OakLog => Some(&BLOCK_OAK_LOG),
        BlockType::SpruceLog => Some(&BLOCK_SPRUCE_LOG),
        BlockType::BirchLog => Some(&BLOCK_BIRCH_LOG),
        BlockType::JungleLog => Some(&BLOCK_JUNGLE_LOG),
        BlockType::AcaciaLog => Some(&BLOCK_ACACIA_LOG),
        BlockType::DarkOakLog => Some(&BLOCK_DARK_OAK_LOG),
        BlockType::StrippedSpruceLog => Some(&BLOCK_STRIPPED_SPRUCE_LOG),
        BlockType::StrippedBirchLog => Some(&BLOCK_STRIPPED_BIRCH_LOG),
        BlockType::StrippedJungleLog => Some(&BLOCK_STRIPPED_JUNGLE_LOG),
        BlockType::StrippedAcaciaLog => Some(&BLOCK_STRIPPED_ACACIA_LOG),
        BlockType::StrippedDarkOakLog => Some(&BLOCK_STRIPPED_DARK_OAK_LOG),
        BlockType::StrippedOakLog => Some(&BLOCK_STRIPPED_OAK_LOG),
        BlockType::OakWood => Some(&BLOCK_OAK_WOOD),
        BlockType::SpruceWood => Some(&BLOCK_SPRUCE_WOOD),
        BlockType::BirchWood => Some(&BLOCK_BIRCH_WOOD),
        BlockType::JungleWood => Some(&BLOCK_JUNGLE_WOOD),
        BlockType::AcaciaWood => Some(&BLOCK_ACACIA_WOOD),
        BlockType::DarkOakWood => Some(&BLOCK_DARK_OAK_WOOD),
        BlockType::StrippedOakWood => Some(&BLOCK_STRIPPED_OAK_WOOD),
        BlockType::StrippedSpruceWood => Some(&BLOCK_STRIPPED_SPRUCE_WOOD),
        BlockType::StrippedBirchWood => Some(&BLOCK_STRIPPED_BIRCH_WOOD),
        BlockType::StrippedJungleWood => Some(&BLOCK_STRIPPED_JUNGLE_WOOD),
        BlockType::StrippedAcaciaWood => Some(&BLOCK_STRIPPED_ACACIA_WOOD),
        BlockType::StrippedDarkOakWood => Some(&BLOCK_STRIPPED_DARK_OAK_WOOD),
        BlockType::OakLeaves => Some(&BLOCK_OAK_LEAVES),
        BlockType::SpruceLeaves => Some(&BLOCK_SPRUCE_LEAVES),
        BlockType::BirchLeaves => Some(&BLOCK_BIRCH_LEAVES),
        BlockType::JungleLeaves => Some(&BLOCK_JUNGLE_LEAVES),
        BlockType::AcaciaLeaves => Some(&BLOCK_ACACIA_LEAVES),
        BlockType::DarkOakLeaves => Some(&BLOCK_DARK_OAK_LEAVES),
        BlockType::Sponge => Some(&BLOCK_SPONGE),
        BlockType::WetSponge => Some(&BLOCK_WET_SPONGE),
        BlockType::Glass => Some(&BLOCK_GLASS),
        BlockType::LapisOre => Some(&BLOCK_LAPIS_ORE),
        BlockType::LapisBlock => Some(&BLOCK_LAPIS_BLOCK),
        BlockType::Dispenser => Some(&BLOCK_DISPENSER),
        BlockType::Sandstone => Some(&BLOCK_SANDSTONE),
        BlockType::ChiseledSandstone => Some(&BLOCK_CHISELED_SANDSTONE),
        BlockType::CutSandstone => Some(&BLOCK_CUT_SANDSTONE),
        BlockType::NoteBlock => Some(&BLOCK_NOTE_BLOCK),
        BlockType::StickyPiston => Some(&BLOCK_STICKY_PISTON),
        BlockType::Piston => Some(&BLOCK_PISTON),
        BlockType::PistonHead => Some(&BLOCK_PISTON_HEAD),
        BlockType::WhiteWool => Some(&BLOCK_WHITE_WOOL),
        BlockType::OrangeWool => Some(&BLOCK_ORANGE_WOOL),
        BlockType::MagentaWool => Some(&BLOCK_MAGENTA_WOOL),
        BlockType::LightBlueWool => Some(&BLOCK_LIGHT_BLUE_WOOL),
        BlockType::YellowWool => Some(&BLOCK_YELLOW_WOOL),
        BlockType::LimeWool => Some(&BLOCK_LIME_WOOL),
        BlockType::PinkWool => Some(&BLOCK_PINK_WOOL),
        BlockType::GrayWool => Some(&BLOCK_GRAY_WOOL),
        BlockType::LightGrayWool => Some(&BLOCK_LIGHT_GRAY_WOOL),
        BlockType::CyanWool => Some(&BLOCK_CYAN_WOOL),
        BlockType::PurpleWool => Some(&BLOCK_PURPLE_WOOL),
        BlockType::BlueWool => Some(&BLOCK_BLUE_WOOL),
        BlockType::BrownWool => Some(&BLOCK_BROWN_WOOL),
        BlockType::GreenWool => Some(&BLOCK_GREEN_WOOL),
        BlockType::RedWool => Some(&BLOCK_RED_WOOL),
        BlockType::BlackWool => Some(&BLOCK_BLACK_WOOL),
        BlockType::GoldBlock => Some(&BLOCK_GOLD_BLOCK),
        BlockType::IronBlock => Some(&BLOCK_IRON_BLOCK),
        BlockType::Bricks => Some(&BLOCK_BRICKS),
        BlockType::Tnt => Some(&BLOCK_TNT),
        BlockType::Bookshelf => Some(&BLOCK_BOOKSHELF),
        BlockType::MossyCobblestone => Some(&BLOCK_MOSSY_COBBLESTONE),
        BlockType::Obsidian => Some(&BLOCK_OBSIDIAN),
        BlockType::Spawner => Some(&BLOCK_SPAWNER),
        BlockType::OakStairs => Some(&BLOCK_OAK_STAIRS),
        BlockType::Chest => Some(&BLOCK_CHEST),
        BlockType::DiamondOre => Some(&BLOCK_DIAMOND_ORE),
        BlockType::DiamondBlock => Some(&BLOCK_DIAMOND_BLOCK),
        BlockType::CraftingTable => Some(&BLOCK_CRAFTING_TABLE),
        BlockType::Farmland => Some(&BLOCK_FARMLAND),
        BlockType::Furnace => Some(&BLOCK_FURNACE),
        BlockType::OakDoor => Some(&BLOCK_OAK_DOOR),
        BlockType::Ladder => Some(&BLOCK_LADDER),
        BlockType::CobblestoneStairs => Some(&BLOCK_COBBLESTONE_STAIRS),
        BlockType::IronDoor => Some(&BLOCK_IRON_DOOR),
        BlockType::RedstoneOre => Some(&BLOCK_REDSTONE_ORE),
        BlockType::Snow => Some(&BLOCK_SNOW),
        BlockType::Ice => Some(&BLOCK_ICE),
        BlockType::SnowBlock => Some(&BLOCK_SNOW_BLOCK),
        BlockType::Cactus => Some(&BLOCK_CACTUS),
        BlockType::Clay => Some(&BLOCK_CLAY),
        BlockType::Jukebox => Some(&BLOCK_JUKEBOX),
        BlockType::OakFence => Some(&BLOCK_OAK_FENCE),
        BlockType::Pumpkin => Some(&BLOCK_PUMPKIN),
        BlockType::Netherrack => Some(&BLOCK_NETHERRACK),
        BlockType::SoulSand => Some(&BLOCK_SOUL_SAND),
        BlockType::SoulSoil => Some(&BLOCK_SOUL_SOIL),
        BlockType::Basalt => Some(&BLOCK_BASALT),
        BlockType::PolishedBasalt => Some(&BLOCK_POLISHED_BASALT),
        BlockType::Glowstone => Some(&BLOCK_GLOWSTONE),
        BlockType::CarvedPumpkin => Some(&BLOCK_CARVED_PUMPKIN),
        BlockType::JackOLantern => Some(&BLOCK_JACK_O_LANTERN),
        BlockType::Cake => Some(&BLOCK_CAKE),
        BlockType::Repeater => Some(&BLOCK_REPEATER),
        BlockType::WhiteStainedGlass => Some(&BLOCK_WHITE_STAINED_GLASS),
        BlockType::OrangeStainedGlass => Some(&BLOCK_ORANGE_STAINED_GLASS),
        BlockType::MagentaStainedGlass => Some(&BLOCK_MAGENTA_STAINED_GLASS),
        BlockType::LightBlueStainedGlass => Some(&BLOCK_LIGHT_BLUE_STAINED_GLASS),
        BlockType::YellowStainedGlass => Some(&BLOCK_YELLOW_STAINED_GLASS),
        BlockType::LimeStainedGlass => Some(&BLOCK_LIME_STAINED_GLASS),
        BlockType::PinkStainedGlass => Some(&BLOCK_PINK_STAINED_GLASS),
        BlockType::GrayStainedGlass => Some(&BLOCK_GRAY_STAINED_GLASS),
        BlockType::LightGrayStainedGlass => Some(&BLOCK_LIGHT_GRAY_STAINED_GLASS),
        BlockType::CyanStainedGlass => Some(&BLOCK_CYAN_STAINED_GLASS),
        BlockType::PurpleStainedGlass => Some(&BLOCK_PURPLE_STAINED_GLASS),
        BlockType::BlueStainedGlass => Some(&BLOCK_BLUE_STAINED_GLASS),
        BlockType::BrownStainedGlass => Some(&BLOCK_BROWN_STAINED_GLASS),
        BlockType::GreenStainedGlass => Some(&BLOCK_GREEN_STAINED_GLASS),
        BlockType::RedStainedGlass => Some(&BLOCK_RED_STAINED_GLASS),
        BlockType::BlackStainedGlass => Some(&BLOCK_BLACK_STAINED_GLASS),
        BlockType::OakTrapdoor => Some(&BLOCK_OAK_TRAPDOOR),
        BlockType::SpruceTrapdoor => Some(&BLOCK_SPRUCE_TRAPDOOR),
        BlockType::BirchTrapdoor => Some(&BLOCK_BIRCH_TRAPDOOR),
        BlockType::JungleTrapdoor => Some(&BLOCK_JUNGLE_TRAPDOOR),
        BlockType::AcaciaTrapdoor => Some(&BLOCK_ACACIA_TRAPDOOR),
        BlockType::DarkOakTrapdoor => Some(&BLOCK_DARK_OAK_TRAPDOOR),
        BlockType::StoneBricks => Some(&BLOCK_STONE_BRICKS),
        BlockType::MossyStoneBricks => Some(&BLOCK_MOSSY_STONE_BRICKS),
        BlockType::CrackedStoneBricks => Some(&BLOCK_CRACKED_STONE_BRICKS),
        BlockType::ChiseledStoneBricks => Some(&BLOCK_CHISELED_STONE_BRICKS),
        BlockType::InfestedStone => Some(&BLOCK_INFESTED_STONE),
        BlockType::InfestedCobblestone => Some(&BLOCK_INFESTED_COBBLESTONE),
        BlockType::InfestedStoneBricks => Some(&BLOCK_INFESTED_STONE_BRICKS),
        BlockType::InfestedMossyStoneBricks => Some(&BLOCK_INFESTED_MOSSY_STONE_BRICKS),
        BlockType::InfestedCrackedStoneBricks => Some(&BLOCK_INFESTED_CRACKED_STONE_BRICKS),
        BlockType::InfestedChiseledStoneBricks => Some(&BLOCK_INFESTED_CHISELED_STONE_BRICKS),
        BlockType::BrownMushroomBlock => Some(&BLOCK_BROWN_MUSHROOM_BLOCK),
        BlockType::RedMushroomBlock => Some(&BLOCK_RED_MUSHROOM_BLOCK),
        BlockType::MushroomStem => Some(&BLOCK_MUSHROOM_STEM),
        BlockType::IronBars => Some(&BLOCK_IRON_BARS),
        BlockType::Chain => Some(&BLOCK_CHAIN),
        BlockType::GlassPane => Some(&BLOCK_GLASS_PANE),
        BlockType::Melon => Some(&BLOCK_MELON),
        BlockType::OakFenceGate => Some(&BLOCK_OAK_FENCE_GATE),
        BlockType::BrickStairs => Some(&BLOCK_BRICK_STAIRS),
        BlockType::StoneBrickStairs => Some(&BLOCK_STONE_BRICK_STAIRS),
        BlockType::Mycelium => Some(&BLOCK_MYCELIUM),
        BlockType::LilyPad => Some(&BLOCK_LILY_PAD),
        BlockType::NetherBricks => Some(&BLOCK_NETHER_BRICKS),
        BlockType::NetherBrickFence => Some(&BLOCK_NETHER_BRICK_FENCE),
        BlockType::NetherBrickStairs => Some(&BLOCK_NETHER_BRICK_STAIRS),
        BlockType::EnchantingTable => Some(&BLOCK_ENCHANTING_TABLE),
        BlockType::BrewingStand => Some(&BLOCK_BREWING_STAND),
        BlockType::Cauldron => Some(&BLOCK_CAULDRON),
        BlockType::EndPortalFrame => Some(&BLOCK_END_PORTAL_FRAME),
        BlockType::EndStone => Some(&BLOCK_END_STONE),
        BlockType::DragonEgg => Some(&BLOCK_DRAGON_EGG),
        BlockType::RedstoneLamp => Some(&BLOCK_REDSTONE_LAMP),
        BlockType::Cocoa => Some(&BLOCK_COCOA),
        BlockType::SandstoneStairs => Some(&BLOCK_SANDSTONE_STAIRS),
        BlockType::EmeraldOre => Some(&BLOCK_EMERALD_ORE),
        BlockType::EnderChest => Some(&BLOCK_ENDER_CHEST),
        BlockType::EmeraldBlock => Some(&BLOCK_EMERALD_BLOCK),
        BlockType::SpruceStairs => Some(&BLOCK_SPRUCE_STAIRS),
        BlockType::BirchStairs => Some(&BLOCK_BIRCH_STAIRS),
        BlockType::JungleStairs => Some(&BLOCK_JUNGLE_STAIRS),
        BlockType::CommandBlock => Some(&BLOCK_COMMAND_BLOCK),
        BlockType::Beacon => Some(&BLOCK_BEACON),
        BlockType::CobblestoneWall => Some(&BLOCK_COBBLESTONE_WALL),
        BlockType::MossyCobblestoneWall => Some(&BLOCK_MOSSY_COBBLESTONE_WALL),
        BlockType::FlowerPot => Some(&BLOCK_FLOWER_POT),
        BlockType::PottedOakSapling => Some(&BLOCK_POTTED_OAK_SAPLING),
        BlockType::PottedSpruceSapling => Some(&BLOCK_POTTED_SPRUCE_SAPLING),
        BlockType::PottedBirchSapling => Some(&BLOCK_POTTED_BIRCH_SAPLING),
        BlockType::PottedJungleSapling => Some(&BLOCK_POTTED_JUNGLE_SAPLING),
        BlockType::PottedAcaciaSapling => Some(&BLOCK_POTTED_ACACIA_SAPLING),
        BlockType::PottedDarkOakSapling => Some(&BLOCK_POTTED_DARK_OAK_SAPLING),
        BlockType::PottedFern => Some(&BLOCK_POTTED_FERN),
        BlockType::PottedDandelion => Some(&BLOCK_POTTED_DANDELION),
        BlockType::PottedPoppy => Some(&BLOCK_POTTED_POPPY),
        BlockType::PottedBlueOrchid => Some(&BLOCK_POTTED_BLUE_ORCHID),
        BlockType::PottedAllium => Some(&BLOCK_POTTED_ALLIUM),
        BlockType::PottedAzureBluet => Some(&BLOCK_POTTED_AZURE_BLUET),
        BlockType::PottedRedTulip => Some(&BLOCK_POTTED_RED_TULIP),
        BlockType::PottedOrangeTulip => Some(&BLOCK_POTTED_ORANGE_TULIP),
        BlockType::PottedWhiteTulip => Some(&BLOCK_POTTED_WHITE_TULIP),
        BlockType::PottedPinkTulip => Some(&BLOCK_POTTED_PINK_TULIP),
        BlockType::PottedOxeyeDaisy => Some(&BLOCK_POTTED_OXEYE_DAISY),
        BlockType::PottedCornflower => Some(&BLOCK_POTTED_CORNFLOWER),
        BlockType::PottedLilyOfTheValley => Some(&BLOCK_POTTED_LILY_OF_THE_VALLEY),
        BlockType::PottedWitherRose => Some(&BLOCK_POTTED_WITHER_ROSE),
        BlockType::PottedRedMushroom => Some(&BLOCK_POTTED_RED_MUSHROOM),
        BlockType::PottedBrownMushroom => Some(&BLOCK_POTTED_BROWN_MUSHROOM),
        BlockType::PottedDeadBush => Some(&BLOCK_POTTED_DEAD_BUSH),
        BlockType::PottedCactus => Some(&BLOCK_POTTED_CACTUS),
        BlockType::SkeletonSkull => Some(&BLOCK_SKELETON_SKULL),
        BlockType::SkeletonWallSkull => Some(&BLOCK_SKELETON_WALL_SKULL),
        BlockType::WitherSkeletonSkull => Some(&BLOCK_WITHER_SKELETON_SKULL),
        BlockType::WitherSkeletonWallSkull => Some(&BLOCK_WITHER_SKELETON_WALL_SKULL),
        BlockType::ZombieHead => Some(&BLOCK_ZOMBIE_HEAD),
        BlockType::ZombieWallHead => Some(&BLOCK_ZOMBIE_WALL_HEAD),
        BlockType::PlayerHead => Some(&BLOCK_PLAYER_HEAD),
        BlockType::PlayerWallHead => Some(&BLOCK_PLAYER_WALL_HEAD),
        BlockType::CreeperHead => Some(&BLOCK_CREEPER_HEAD),
        BlockType::CreeperWallHead => Some(&BLOCK_CREEPER_WALL_HEAD),
        BlockType::DragonHead => Some(&BLOCK_DRAGON_HEAD),
        BlockType::DragonWallHead => Some(&BLOCK_DRAGON_WALL_HEAD),
        BlockType::Anvil => Some(&BLOCK_ANVIL),
        BlockType::ChippedAnvil => Some(&BLOCK_CHIPPED_ANVIL),
        BlockType::DamagedAnvil => Some(&BLOCK_DAMAGED_ANVIL),
        BlockType::TrappedChest => Some(&BLOCK_TRAPPED_CHEST),
        BlockType::Comparator => Some(&BLOCK_COMPARATOR),
        BlockType::DaylightDetector => Some(&BLOCK_DAYLIGHT_DETECTOR),
        BlockType::RedstoneBlock => Some(&BLOCK_REDSTONE_BLOCK),
        BlockType::NetherQuartzOre => Some(&BLOCK_NETHER_QUARTZ_ORE),
        BlockType::Hopper => Some(&BLOCK_HOPPER),
        BlockType::QuartzBlock => Some(&BLOCK_QUARTZ_BLOCK),
        BlockType::ChiseledQuartzBlock => Some(&BLOCK_CHISELED_QUARTZ_BLOCK),
        BlockType::QuartzPillar => Some(&BLOCK_QUARTZ_PILLAR),
        BlockType::QuartzStairs => Some(&BLOCK_QUARTZ_STAIRS),
        BlockType::Dropper => Some(&BLOCK_DROPPER),
        BlockType::WhiteTerracotta => Some(&BLOCK_WHITE_TERRACOTTA),
        BlockType::OrangeTerracotta => Some(&BLOCK_ORANGE_TERRACOTTA),
        BlockType::MagentaTerracotta => Some(&BLOCK_MAGENTA_TERRACOTTA),
        BlockType::LightBlueTerracotta => Some(&BLOCK_LIGHT_BLUE_TERRACOTTA),
        BlockType::YellowTerracotta => Some(&BLOCK_YELLOW_TERRACOTTA),
        BlockType::LimeTerracotta => Some(&BLOCK_LIME_TERRACOTTA),
        BlockType::PinkTerracotta => Some(&BLOCK_PINK_TERRACOTTA),
        BlockType::GrayTerracotta => Some(&BLOCK_GRAY_TERRACOTTA),
        BlockType::LightGrayTerracotta => Some(&BLOCK_LIGHT_GRAY_TERRACOTTA),
        BlockType::CyanTerracotta => Some(&BLOCK_CYAN_TERRACOTTA),
        BlockType::PurpleTerracotta => Some(&BLOCK_PURPLE_TERRACOTTA),
        BlockType::BlueTerracotta => Some(&BLOCK_BLUE_TERRACOTTA),
        BlockType::BrownTerracotta => Some(&BLOCK_BROWN_TERRACOTTA),
        BlockType::GreenTerracotta => Some(&BLOCK_GREEN_TERRACOTTA),
        BlockType::RedTerracotta => Some(&BLOCK_RED_TERRACOTTA),
        BlockType::BlackTerracotta => Some(&BLOCK_BLACK_TERRACOTTA),
        BlockType::WhiteStainedGlassPane => Some(&BLOCK_WHITE_STAINED_GLASS_PANE),
        BlockType::OrangeStainedGlassPane => Some(&BLOCK_ORANGE_STAINED_GLASS_PANE),
        BlockType::MagentaStainedGlassPane => Some(&BLOCK_MAGENTA_STAINED_GLASS_PANE),
        BlockType::LightBlueStainedGlassPane => Some(&BLOCK_LIGHT_BLUE_STAINED_GLASS_PANE),
        BlockType::YellowStainedGlassPane => Some(&BLOCK_YELLOW_STAINED_GLASS_PANE),
        BlockType::LimeStainedGlassPane => Some(&BLOCK_LIME_STAINED_GLASS_PANE),
        BlockType::PinkStainedGlassPane => Some(&BLOCK_PINK_STAINED_GLASS_PANE),
        BlockType::GrayStainedGlassPane => Some(&BLOCK_GRAY_STAINED_GLASS_PANE),
        BlockType::LightGrayStainedGlassPane => Some(&BLOCK_LIGHT_GRAY_STAINED_GLASS_PANE),
        BlockType::CyanStainedGlassPane => Some(&BLOCK_CYAN_STAINED_GLASS_PANE),
        BlockType::PurpleStainedGlassPane => Some(&BLOCK_PURPLE_STAINED_GLASS_PANE),
        BlockType::BlueStainedGlassPane => Some(&BLOCK_BLUE_STAINED_GLASS_PANE),
        BlockType::BrownStainedGlassPane => Some(&BLOCK_BROWN_STAINED_GLASS_PANE),
        BlockType::GreenStainedGlassPane => Some(&BLOCK_GREEN_STAINED_GLASS_PANE),
        BlockType::RedStainedGlassPane => Some(&BLOCK_RED_STAINED_GLASS_PANE),
        BlockType::BlackStainedGlassPane => Some(&BLOCK_BLACK_STAINED_GLASS_PANE),
        BlockType::AcaciaStairs => Some(&BLOCK_ACACIA_STAIRS),
        BlockType::DarkOakStairs => Some(&BLOCK_DARK_OAK_STAIRS),
        BlockType::SlimeBlock => Some(&BLOCK_SLIME_BLOCK),
        // BlockType::Barrier => Some(&BLOCK_BARRIER),
        BlockType::IronTrapdoor => Some(&BLOCK_IRON_TRAPDOOR),
        BlockType::Prismarine => Some(&BLOCK_PRISMARINE),
        BlockType::PrismarineBricks => Some(&BLOCK_PRISMARINE_BRICKS),
        BlockType::DarkPrismarine => Some(&BLOCK_DARK_PRISMARINE),
        BlockType::PrismarineStairs => Some(&BLOCK_PRISMARINE_STAIRS),
        BlockType::PrismarineBrickStairs => Some(&BLOCK_PRISMARINE_BRICK_STAIRS),
        BlockType::DarkPrismarineStairs => Some(&BLOCK_DARK_PRISMARINE_STAIRS),
        BlockType::PrismarineSlab => Some(&BLOCK_PRISMARINE_SLAB),
        BlockType::PrismarineBrickSlab => Some(&BLOCK_PRISMARINE_BRICK_SLAB),
        BlockType::DarkPrismarineSlab => Some(&BLOCK_DARK_PRISMARINE_SLAB),
        BlockType::SeaLantern => Some(&BLOCK_SEA_LANTERN),
        BlockType::HayBlock => Some(&BLOCK_HAY_BLOCK),
        BlockType::WhiteCarpet => Some(&BLOCK_WHITE_CARPET),
        BlockType::OrangeCarpet => Some(&BLOCK_ORANGE_CARPET),
        BlockType::MagentaCarpet => Some(&BLOCK_MAGENTA_CARPET),
        BlockType::LightBlueCarpet => Some(&BLOCK_LIGHT_BLUE_CARPET),
        BlockType::YellowCarpet => Some(&BLOCK_YELLOW_CARPET),
        BlockType::LimeCarpet => Some(&BLOCK_LIME_CARPET),
        BlockType::PinkCarpet => Some(&BLOCK_PINK_CARPET),
        BlockType::GrayCarpet => Some(&BLOCK_GRAY_CARPET),
        BlockType::LightGrayCarpet => Some(&BLOCK_LIGHT_GRAY_CARPET),
        BlockType::CyanCarpet => Some(&BLOCK_CYAN_CARPET),
        BlockType::PurpleCarpet => Some(&BLOCK_PURPLE_CARPET),
        BlockType::BlueCarpet => Some(&BLOCK_BLUE_CARPET),
        BlockType::BrownCarpet => Some(&BLOCK_BROWN_CARPET),
        BlockType::GreenCarpet => Some(&BLOCK_GREEN_CARPET),
        BlockType::RedCarpet => Some(&BLOCK_RED_CARPET),
        BlockType::BlackCarpet => Some(&BLOCK_BLACK_CARPET),
        BlockType::Terracotta => Some(&BLOCK_TERRACOTTA),
        BlockType::CoalBlock => Some(&BLOCK_COAL_BLOCK),
        BlockType::PackedIce => Some(&BLOCK_PACKED_ICE),
        BlockType::RedSandstone => Some(&BLOCK_RED_SANDSTONE),
        BlockType::ChiseledRedSandstone => Some(&BLOCK_CHISELED_RED_SANDSTONE),
        BlockType::CutRedSandstone => Some(&BLOCK_CUT_RED_SANDSTONE),
        BlockType::RedSandstoneStairs => Some(&BLOCK_RED_SANDSTONE_STAIRS),
        BlockType::OakSlab => Some(&BLOCK_OAK_SLAB),
        BlockType::SpruceSlab => Some(&BLOCK_SPRUCE_SLAB),
        BlockType::BirchSlab => Some(&BLOCK_BIRCH_SLAB),
        BlockType::JungleSlab => Some(&BLOCK_JUNGLE_SLAB),
        BlockType::AcaciaSlab => Some(&BLOCK_ACACIA_SLAB),
        BlockType::DarkOakSlab => Some(&BLOCK_DARK_OAK_SLAB),
        BlockType::StoneSlab => Some(&BLOCK_STONE_SLAB),
        BlockType::SmoothStoneSlab => Some(&BLOCK_SMOOTH_STONE_SLAB),
        BlockType::SandstoneSlab => Some(&BLOCK_SANDSTONE_SLAB),
        BlockType::CutSandstoneSlab => Some(&BLOCK_CUT_SANDSTONE_SLAB),
        BlockType::PetrifiedOakSlab => Some(&BLOCK_PETRIFIED_OAK_SLAB),
        BlockType::CobblestoneSlab => Some(&BLOCK_COBBLESTONE_SLAB),
        BlockType::BrickSlab => Some(&BLOCK_BRICK_SLAB),
        BlockType::StoneBrickSlab => Some(&BLOCK_STONE_BRICK_SLAB),
        BlockType::NetherBrickSlab => Some(&BLOCK_NETHER_BRICK_SLAB),
        BlockType::QuartzSlab => Some(&BLOCK_QUARTZ_SLAB),
        BlockType::RedSandstoneSlab => Some(&BLOCK_RED_SANDSTONE_SLAB),
        BlockType::CutRedSandstoneSlab => Some(&BLOCK_CUT_RED_SANDSTONE_SLAB),
        BlockType::PurpurSlab => Some(&BLOCK_PURPUR_SLAB),
        BlockType::SmoothStone => Some(&BLOCK_SMOOTH_STONE),
        BlockType::SmoothSandstone => Some(&BLOCK_SMOOTH_SANDSTONE),
        BlockType::SmoothQuartz => Some(&BLOCK_SMOOTH_QUARTZ),
        BlockType::SmoothRedSandstone => Some(&BLOCK_SMOOTH_RED_SANDSTONE),
        BlockType::SpruceFenceGate => Some(&BLOCK_SPRUCE_FENCE_GATE),
        BlockType::BirchFenceGate => Some(&BLOCK_BIRCH_FENCE_GATE),
        BlockType::JungleFenceGate => Some(&BLOCK_JUNGLE_FENCE_GATE),
        BlockType::AcaciaFenceGate => Some(&BLOCK_ACACIA_FENCE_GATE),
        BlockType::DarkOakFenceGate => Some(&BLOCK_DARK_OAK_FENCE_GATE),
        BlockType::SpruceFence => Some(&BLOCK_SPRUCE_FENCE),
        BlockType::BirchFence => Some(&BLOCK_BIRCH_FENCE),
        BlockType::JungleFence => Some(&BLOCK_JUNGLE_FENCE),
        BlockType::AcaciaFence => Some(&BLOCK_ACACIA_FENCE),
        BlockType::DarkOakFence => Some(&BLOCK_DARK_OAK_FENCE),
        BlockType::SpruceDoor => Some(&BLOCK_SPRUCE_DOOR),
        BlockType::BirchDoor => Some(&BLOCK_BIRCH_DOOR),
        BlockType::JungleDoor => Some(&BLOCK_JUNGLE_DOOR),
        BlockType::AcaciaDoor => Some(&BLOCK_ACACIA_DOOR),
        BlockType::DarkOakDoor => Some(&BLOCK_DARK_OAK_DOOR),
        BlockType::EndRod => Some(&BLOCK_END_ROD),
        BlockType::ChorusPlant => Some(&BLOCK_CHORUS_PLANT),
        BlockType::ChorusFlower => Some(&BLOCK_CHORUS_FLOWER),
        BlockType::PurpurBlock => Some(&BLOCK_PURPUR_BLOCK),
        BlockType::PurpurPillar => Some(&BLOCK_PURPUR_PILLAR),
        BlockType::PurpurStairs => Some(&BLOCK_PURPUR_STAIRS),
        BlockType::EndStoneBricks => Some(&BLOCK_END_STONE_BRICKS),
        BlockType::GrassPath => Some(&BLOCK_GRASS_PATH),
        BlockType::RepeatingCommandBlock => Some(&BLOCK_REPEATING_COMMAND_BLOCK),
        BlockType::ChainCommandBlock => Some(&BLOCK_CHAIN_COMMAND_BLOCK),
        BlockType::FrostedIce => Some(&BLOCK_FROSTED_ICE),
        BlockType::MagmaBlock => Some(&BLOCK_MAGMA_BLOCK),
        BlockType::NetherWartBlock => Some(&BLOCK_NETHER_WART_BLOCK),
        BlockType::RedNetherBricks => Some(&BLOCK_RED_NETHER_BRICKS),
        BlockType::BoneBlock => Some(&BLOCK_BONE_BLOCK),
        BlockType::Observer => Some(&BLOCK_OBSERVER),
        BlockType::ShulkerBox => Some(&BLOCK_SHULKER_BOX),
        BlockType::WhiteShulkerBox => Some(&BLOCK_WHITE_SHULKER_BOX),
        BlockType::OrangeShulkerBox => Some(&BLOCK_ORANGE_SHULKER_BOX),
        BlockType::MagentaShulkerBox => Some(&BLOCK_MAGENTA_SHULKER_BOX),
        BlockType::LightBlueShulkerBox => Some(&BLOCK_LIGHT_BLUE_SHULKER_BOX),
        BlockType::YellowShulkerBox => Some(&BLOCK_YELLOW_SHULKER_BOX),
        BlockType::LimeShulkerBox => Some(&BLOCK_LIME_SHULKER_BOX),
        BlockType::PinkShulkerBox => Some(&BLOCK_PINK_SHULKER_BOX),
        BlockType::GrayShulkerBox => Some(&BLOCK_GRAY_SHULKER_BOX),
        BlockType::LightGrayShulkerBox => Some(&BLOCK_LIGHT_GRAY_SHULKER_BOX),
        BlockType::CyanShulkerBox => Some(&BLOCK_CYAN_SHULKER_BOX),
        BlockType::PurpleShulkerBox => Some(&BLOCK_PURPLE_SHULKER_BOX),
        BlockType::BlueShulkerBox => Some(&BLOCK_BLUE_SHULKER_BOX),
        BlockType::BrownShulkerBox => Some(&BLOCK_BROWN_SHULKER_BOX),
        BlockType::GreenShulkerBox => Some(&BLOCK_GREEN_SHULKER_BOX),
        BlockType::RedShulkerBox => Some(&BLOCK_RED_SHULKER_BOX),
        BlockType::BlackShulkerBox => Some(&BLOCK_BLACK_SHULKER_BOX),
        BlockType::WhiteGlazedTerracotta => Some(&BLOCK_WHITE_GLAZED_TERRACOTTA),
        BlockType::OrangeGlazedTerracotta => Some(&BLOCK_ORANGE_GLAZED_TERRACOTTA),
        BlockType::MagentaGlazedTerracotta => Some(&BLOCK_MAGENTA_GLAZED_TERRACOTTA),
        BlockType::LightBlueGlazedTerracotta => Some(&BLOCK_LIGHT_BLUE_GLAZED_TERRACOTTA),
        BlockType::YellowGlazedTerracotta => Some(&BLOCK_YELLOW_GLAZED_TERRACOTTA),
        BlockType::LimeGlazedTerracotta => Some(&BLOCK_LIME_GLAZED_TERRACOTTA),
        BlockType::PinkGlazedTerracotta => Some(&BLOCK_PINK_GLAZED_TERRACOTTA),
        BlockType::GrayGlazedTerracotta => Some(&BLOCK_GRAY_GLAZED_TERRACOTTA),
        BlockType::LightGrayGlazedTerracotta => Some(&BLOCK_LIGHT_GRAY_GLAZED_TERRACOTTA),
        BlockType::CyanGlazedTerracotta => Some(&BLOCK_CYAN_GLAZED_TERRACOTTA),
        BlockType::PurpleGlazedTerracotta => Some(&BLOCK_PURPLE_GLAZED_TERRACOTTA),
        BlockType::BlueGlazedTerracotta => Some(&BLOCK_BLUE_GLAZED_TERRACOTTA),
        BlockType::BrownGlazedTerracotta => Some(&BLOCK_BROWN_GLAZED_TERRACOTTA),
        BlockType::GreenGlazedTerracotta => Some(&BLOCK_GREEN_GLAZED_TERRACOTTA),
        BlockType::RedGlazedTerracotta => Some(&BLOCK_RED_GLAZED_TERRACOTTA),
        BlockType::BlackGlazedTerracotta => Some(&BLOCK_BLACK_GLAZED_TERRACOTTA),
        BlockType::WhiteConcrete => Some(&BLOCK_WHITE_CONCRETE),
        BlockType::OrangeConcrete => Some(&BLOCK_ORANGE_CONCRETE),
        BlockType::MagentaConcrete => Some(&BLOCK_MAGENTA_CONCRETE),
        BlockType::LightBlueConcrete => Some(&BLOCK_LIGHT_BLUE_CONCRETE),
        BlockType::YellowConcrete => Some(&BLOCK_YELLOW_CONCRETE),
        BlockType::LimeConcrete => Some(&BLOCK_LIME_CONCRETE),
        BlockType::PinkConcrete => Some(&BLOCK_PINK_CONCRETE),
        BlockType::GrayConcrete => Some(&BLOCK_GRAY_CONCRETE),
        BlockType::LightGrayConcrete => Some(&BLOCK_LIGHT_GRAY_CONCRETE),
        BlockType::CyanConcrete => Some(&BLOCK_CYAN_CONCRETE),
        BlockType::PurpleConcrete => Some(&BLOCK_PURPLE_CONCRETE),
        BlockType::BlueConcrete => Some(&BLOCK_BLUE_CONCRETE),
        BlockType::BrownConcrete => Some(&BLOCK_BROWN_CONCRETE),
        BlockType::GreenConcrete => Some(&BLOCK_GREEN_CONCRETE),
        BlockType::RedConcrete => Some(&BLOCK_RED_CONCRETE),
        BlockType::BlackConcrete => Some(&BLOCK_BLACK_CONCRETE),
        BlockType::WhiteConcretePowder => Some(&BLOCK_WHITE_CONCRETE_POWDER),
        BlockType::OrangeConcretePowder => Some(&BLOCK_ORANGE_CONCRETE_POWDER),
        BlockType::MagentaConcretePowder => Some(&BLOCK_MAGENTA_CONCRETE_POWDER),
        BlockType::LightBlueConcretePowder => Some(&BLOCK_LIGHT_BLUE_CONCRETE_POWDER),
        BlockType::YellowConcretePowder => Some(&BLOCK_YELLOW_CONCRETE_POWDER),
        BlockType::LimeConcretePowder => Some(&BLOCK_LIME_CONCRETE_POWDER),
        BlockType::PinkConcretePowder => Some(&BLOCK_PINK_CONCRETE_POWDER),
        BlockType::GrayConcretePowder => Some(&BLOCK_GRAY_CONCRETE_POWDER),
        BlockType::LightGrayConcretePowder => Some(&BLOCK_LIGHT_GRAY_CONCRETE_POWDER),
        BlockType::CyanConcretePowder => Some(&BLOCK_CYAN_CONCRETE_POWDER),
        BlockType::PurpleConcretePowder => Some(&BLOCK_PURPLE_CONCRETE_POWDER),
        BlockType::BlueConcretePowder => Some(&BLOCK_BLUE_CONCRETE_POWDER),
        BlockType::BrownConcretePowder => Some(&BLOCK_BROWN_CONCRETE_POWDER),
        BlockType::GreenConcretePowder => Some(&BLOCK_GREEN_CONCRETE_POWDER),
        BlockType::RedConcretePowder => Some(&BLOCK_RED_CONCRETE_POWDER),
        BlockType::BlackConcretePowder => Some(&BLOCK_BLACK_CONCRETE_POWDER),
        BlockType::DriedKelpBlock => Some(&BLOCK_DRIED_KELP_BLOCK),
        BlockType::TurtleEgg => Some(&BLOCK_TURTLE_EGG),
        BlockType::DeadTubeCoralBlock => Some(&BLOCK_DEAD_TUBE_CORAL_BLOCK),
        BlockType::DeadBrainCoralBlock => Some(&BLOCK_DEAD_BRAIN_CORAL_BLOCK),
        BlockType::DeadBubbleCoralBlock => Some(&BLOCK_DEAD_BUBBLE_CORAL_BLOCK),
        BlockType::DeadFireCoralBlock => Some(&BLOCK_DEAD_FIRE_CORAL_BLOCK),
        BlockType::DeadHornCoralBlock => Some(&BLOCK_DEAD_HORN_CORAL_BLOCK),
        BlockType::TubeCoralBlock => Some(&BLOCK_TUBE_CORAL_BLOCK),
        BlockType::BrainCoralBlock => Some(&BLOCK_BRAIN_CORAL_BLOCK),
        BlockType::BubbleCoralBlock => Some(&BLOCK_BUBBLE_CORAL_BLOCK),
        BlockType::FireCoralBlock => Some(&BLOCK_FIRE_CORAL_BLOCK),
        BlockType::HornCoralBlock => Some(&BLOCK_HORN_CORAL_BLOCK),
        BlockType::SeaPickle => Some(&BLOCK_SEA_PICKLE),
        BlockType::BlueIce => Some(&BLOCK_BLUE_ICE),
        BlockType::Conduit => Some(&BLOCK_CONDUIT),
        BlockType::Bamboo => Some(&BLOCK_BAMBOO),
        BlockType::PottedBamboo => Some(&BLOCK_POTTED_BAMBOO),
        BlockType::PolishedGraniteStairs => Some(&BLOCK_POLISHED_GRANITE_STAIRS),
        BlockType::SmoothRedSandstoneStairs => Some(&BLOCK_SMOOTH_RED_SANDSTONE_STAIRS),
        BlockType::MossyStoneBrickStairs => Some(&BLOCK_MOSSY_STONE_BRICK_STAIRS),
        BlockType::PolishedDioriteStairs => Some(&BLOCK_POLISHED_DIORITE_STAIRS),
        BlockType::MossyCobblestoneStairs => Some(&BLOCK_MOSSY_COBBLESTONE_STAIRS),
        BlockType::EndStoneBrickStairs => Some(&BLOCK_END_STONE_BRICK_STAIRS),
        BlockType::StoneStairs => Some(&BLOCK_STONE_STAIRS),
        BlockType::SmoothSandstoneStairs => Some(&BLOCK_SMOOTH_SANDSTONE_STAIRS),
        BlockType::SmoothQuartzStairs => Some(&BLOCK_SMOOTH_QUARTZ_STAIRS),
        BlockType::GraniteStairs => Some(&BLOCK_GRANITE_STAIRS),
        BlockType::AndesiteStairs => Some(&BLOCK_ANDESITE_STAIRS),
        BlockType::RedNetherBrickStairs => Some(&BLOCK_RED_NETHER_BRICK_STAIRS),
        BlockType::PolishedAndesiteStairs => Some(&BLOCK_POLISHED_ANDESITE_STAIRS),
        BlockType::DioriteStairs => Some(&BLOCK_DIORITE_STAIRS),
        BlockType::PolishedGraniteSlab => Some(&BLOCK_POLISHED_GRANITE_SLAB),
        BlockType::SmoothRedSandstoneSlab => Some(&BLOCK_SMOOTH_RED_SANDSTONE_SLAB),
        BlockType::MossyStoneBrickSlab => Some(&BLOCK_MOSSY_STONE_BRICK_SLAB),
        BlockType::PolishedDioriteSlab => Some(&BLOCK_POLISHED_DIORITE_SLAB),
        BlockType::MossyCobblestoneSlab => Some(&BLOCK_MOSSY_COBBLESTONE_SLAB),
        BlockType::EndStoneBrickSlab => Some(&BLOCK_END_STONE_BRICK_SLAB),
        BlockType::SmoothSandstoneSlab => Some(&BLOCK_SMOOTH_SANDSTONE_SLAB),
        BlockType::SmoothQuartzSlab => Some(&BLOCK_SMOOTH_QUARTZ_SLAB),
        BlockType::GraniteSlab => Some(&BLOCK_GRANITE_SLAB),
        BlockType::AndesiteSlab => Some(&BLOCK_ANDESITE_SLAB),
        BlockType::RedNetherBrickSlab => Some(&BLOCK_RED_NETHER_BRICK_SLAB),
        BlockType::PolishedAndesiteSlab => Some(&BLOCK_POLISHED_ANDESITE_SLAB),
        BlockType::DioriteSlab => Some(&BLOCK_DIORITE_SLAB),
        BlockType::BrickWall => Some(&BLOCK_BRICK_WALL),
        BlockType::PrismarineWall => Some(&BLOCK_PRISMARINE_WALL),
        BlockType::RedSandstoneWall => Some(&BLOCK_RED_SANDSTONE_WALL),
        BlockType::MossyStoneBrickWall => Some(&BLOCK_MOSSY_STONE_BRICK_WALL),
        BlockType::GraniteWall => Some(&BLOCK_GRANITE_WALL),
        BlockType::StoneBrickWall => Some(&BLOCK_STONE_BRICK_WALL),
        BlockType::NetherBrickWall => Some(&BLOCK_NETHER_BRICK_WALL),
        BlockType::AndesiteWall => Some(&BLOCK_ANDESITE_WALL),
        BlockType::RedNetherBrickWall => Some(&BLOCK_RED_NETHER_BRICK_WALL),
        BlockType::SandstoneWall => Some(&BLOCK_SANDSTONE_WALL),
        BlockType::EndStoneBrickWall => Some(&BLOCK_END_STONE_BRICK_WALL),
        BlockType::DioriteWall => Some(&BLOCK_DIORITE_WALL),
        BlockType::Scaffolding => Some(&BLOCK_SCAFFOLDING),
        BlockType::Loom => Some(&BLOCK_LOOM),
        BlockType::Barrel => Some(&BLOCK_BARREL),
        BlockType::Smoker => Some(&BLOCK_SMOKER),
        BlockType::BlastFurnace => Some(&BLOCK_BLAST_FURNACE),
        BlockType::CartographyTable => Some(&BLOCK_CARTOGRAPHY_TABLE),
        BlockType::FletchingTable => Some(&BLOCK_FLETCHING_TABLE),
        BlockType::Grindstone => Some(&BLOCK_GRINDSTONE),
        BlockType::Lectern => Some(&BLOCK_LECTERN),
        BlockType::SmithingTable => Some(&BLOCK_SMITHING_TABLE),
        BlockType::Stonecutter => Some(&BLOCK_STONECUTTER),
        BlockType::Bell => Some(&BLOCK_BELL),
        BlockType::Lantern => Some(&BLOCK_LANTERN),
        BlockType::SoulLantern => Some(&BLOCK_SOUL_LANTERN),
        BlockType::Campfire => Some(&BLOCK_CAMPFIRE),
        BlockType::SoulCampfire => Some(&BLOCK_SOUL_CAMPFIRE),
        BlockType::WarpedStem => Some(&BLOCK_WARPED_STEM),
        BlockType::StrippedWarpedStem => Some(&BLOCK_STRIPPED_WARPED_STEM),
        BlockType::WarpedHyphae => Some(&BLOCK_WARPED_HYPHAE),
        BlockType::StrippedWarpedHyphae => Some(&BLOCK_STRIPPED_WARPED_HYPHAE),
        BlockType::WarpedNylium => Some(&BLOCK_WARPED_NYLIUM),
        BlockType::WarpedWartBlock => Some(&BLOCK_WARPED_WART_BLOCK),
        BlockType::CrimsonStem => Some(&BLOCK_CRIMSON_STEM),
        BlockType::StrippedCrimsonStem => Some(&BLOCK_STRIPPED_CRIMSON_STEM),
        BlockType::CrimsonHyphae => Some(&BLOCK_CRIMSON_HYPHAE),
        BlockType::StrippedCrimsonHyphae => Some(&BLOCK_STRIPPED_CRIMSON_HYPHAE),
        BlockType::CrimsonNylium => Some(&BLOCK_CRIMSON_NYLIUM),
        BlockType::Shroomlight => Some(&BLOCK_SHROOMLIGHT),
        BlockType::CrimsonPlanks => Some(&BLOCK_CRIMSON_PLANKS),
        BlockType::WarpedPlanks => Some(&BLOCK_WARPED_PLANKS),
        BlockType::CrimsonSlab => Some(&BLOCK_CRIMSON_SLAB),
        BlockType::WarpedSlab => Some(&BLOCK_WARPED_SLAB),
        BlockType::CrimsonFence => Some(&BLOCK_CRIMSON_FENCE),
        BlockType::WarpedFence => Some(&BLOCK_WARPED_FENCE),
        BlockType::CrimsonTrapdoor => Some(&BLOCK_CRIMSON_TRAPDOOR),
        BlockType::WarpedTrapdoor => Some(&BLOCK_WARPED_TRAPDOOR),
        BlockType::CrimsonFenceGate => Some(&BLOCK_CRIMSON_FENCE_GATE),
        BlockType::WarpedFenceGate => Some(&BLOCK_WARPED_FENCE_GATE),
        BlockType::CrimsonStairs => Some(&BLOCK_CRIMSON_STAIRS),
        BlockType::WarpedStairs => Some(&BLOCK_WARPED_STAIRS),
        BlockType::CrimsonDoor => Some(&BLOCK_CRIMSON_DOOR),
        BlockType::WarpedDoor => Some(&BLOCK_WARPED_DOOR),
        BlockType::StructureBlock => Some(&BLOCK_STRUCTURE_BLOCK),
        BlockType::Jigsaw => Some(&BLOCK_JIGSAW),
        BlockType::Composter => Some(&BLOCK_COMPOSTER),
        BlockType::Target => Some(&BLOCK_TARGET),
        BlockType::BeeNest => Some(&BLOCK_BEE_NEST),
        BlockType::Beehive => Some(&BLOCK_BEEHIVE),
        BlockType::HoneyBlock => Some(&BLOCK_HONEY_BLOCK),
        BlockType::HoneycombBlock => Some(&BLOCK_HONEYCOMB_BLOCK),
        BlockType::NetheriteBlock => Some(&BLOCK_NETHERITE_BLOCK),
        BlockType::AncientDebris => Some(&BLOCK_ANCIENT_DEBRIS),
        BlockType::CryingObsidian => Some(&BLOCK_CRYING_OBSIDIAN),
        BlockType::RespawnAnchor => Some(&BLOCK_RESPAWN_ANCHOR),
        BlockType::PottedCrimsonFungus => Some(&BLOCK_POTTED_CRIMSON_FUNGUS),
        BlockType::PottedWarpedFungus => Some(&BLOCK_POTTED_WARPED_FUNGUS),
        BlockType::PottedCrimsonRoots => Some(&BLOCK_POTTED_CRIMSON_ROOTS),
        BlockType::PottedWarpedRoots => Some(&BLOCK_POTTED_WARPED_ROOTS),
        BlockType::Lodestone => Some(&BLOCK_LODESTONE),
        BlockType::Blackstone => Some(&BLOCK_BLACKSTONE),
        BlockType::BlackstoneStairs => Some(&BLOCK_BLACKSTONE_STAIRS),
        BlockType::BlackstoneWall => Some(&BLOCK_BLACKSTONE_WALL),
        BlockType::BlackstoneSlab => Some(&BLOCK_BLACKSTONE_SLAB),
        BlockType::PolishedBlackstone => Some(&BLOCK_POLISHED_BLACKSTONE),
        BlockType::PolishedBlackstoneBricks => Some(&BLOCK_POLISHED_BLACKSTONE_BRICKS),
        BlockType::CrackedPolishedBlackstoneBricks => Some(&BLOCK_CRACKED_POLISHED_BLACKSTONE_BRICKS),
        BlockType::ChiseledPolishedBlackstone => Some(&BLOCK_CHISELED_POLISHED_BLACKSTONE),
        BlockType::PolishedBlackstoneBrickSlab => Some(&BLOCK_POLISHED_BLACKSTONE_BRICK_SLAB),
        BlockType::PolishedBlackstoneBrickStairs => Some(&BLOCK_POLISHED_BLACKSTONE_BRICK_STAIRS),
        BlockType::PolishedBlackstoneBrickWall => Some(&BLOCK_POLISHED_BLACKSTONE_BRICK_WALL),
        BlockType::GildedBlackstone => Some(&BLOCK_GILDED_BLACKSTONE),
        BlockType::PolishedBlackstoneStairs => Some(&BLOCK_POLISHED_BLACKSTONE_STAIRS),
        BlockType::PolishedBlackstoneSlab => Some(&BLOCK_POLISHED_BLACKSTONE_SLAB),
        BlockType::PolishedBlackstoneWall => Some(&BLOCK_POLISHED_BLACKSTONE_WALL),
        BlockType::ChiseledNetherBricks => Some(&BLOCK_CHISELED_NETHER_BRICKS),
        BlockType::CrackedNetherBricks => Some(&BLOCK_CRACKED_NETHER_BRICKS),
        BlockType::QuartzBricks => Some(&BLOCK_QUARTZ_BRICKS),
    }
}

impl PartialEq for BlockType {
    fn eq(&self, other: &BlockType) -> bool {
        *self as u8 == *other as u8
    }
}
