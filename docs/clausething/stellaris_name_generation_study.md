# ClauseNameSpace: Engineering Study of Stellaris Name Generation

**Study date:** 2026-06-24  
**Primary corpus:** `C:\Users\mvorm\Clauser\Paradox\vanilla`  
**Documentation:** `C:\Users\mvorm\Clauser\Paradox\script_documentation`, the local Stellaris modding summaries, and the local Jomini/ClauseThing references  
**Purpose:** explain how Stellaris defines, generates, packages, selects, stores, and renders names for species, characters, empires, ships, fleets, armies, planets, systems, wars, federations, factions, and related entities; identify what is data-driven versus engine-hardcoded; and derive requirements for a faithful ClauseScript consumer.

---

## 1. Executive conclusion

Stellaris does **not** primarily invent pronounceable names from phonemes, syllables, Markov chains, or another linguistic algorithm. Its naming architecture is a hybrid of four mechanisms:

1. **Authored pools.** Most character, ship, fleet, army, planet, species, star, black-hole, and nebula names are pre-authored tokens stored in ClauseScript configuration files.
2. **Small combinators and fallbacks.** The engine selects among generic and entity-specific pools, joins first and second character names, adds sequential numbers, combines asteroid prefixes and postfixes, and falls back between lists according to documented rules.
3. **Weighted, context-sensitive name grammars.** Empire, war, federation, political-faction, and pre-communications names are assembled from weighted part lists and format templates. Format eligibility and weight depend on government, ethics, civics, war goal, federation type, faction type, country flags, and scoped properties such as species adjective or capital-system name.
4. **Localization-backed rendering.** The selected values are usually localization keys or localization templates, not necessarily final display strings. Scoped localization commands inject live names and grammatical forms. Modern saves may retain abstract name templates, so a faithful external renderer must load the relevant game and active-mod localization data.

The engine supplies RNG, weighted choice, object context, uniqueness bookkeeping, fallback behavior, template expansion, and persistence. ClauseScript supplies nearly all of the creative vocabulary and much of the selection policy. The closed C++ engine means the exact PRNG, seed derivation, retry limits, collision policy, and some unnamed-object defaults cannot be recovered from these text files alone.

The most important engineering distinction is therefore:

> Stellaris is **procedural in selection and composition**, but overwhelmingly **curated in vocabulary**.

No evidence in the available corpus indicates an open-ended phonotactic name generator.

---

## 2. Evidence and confidence

### 2.1 Corpus inventory

The copied vanilla corpus contains:

| Area | Files | Lines | Purpose |
|---|---:|---:|---|
| `common/name_lists/` | 76 | 15,650 | 78 top-level name-list definitions plus the schema README |
| `common/species_names/` | 5 | 3,846 | 555 authored species bundles |
| `common/random_names/` | 7 | 11,145 | celestial pools and weighted grammar definitions |
| `common/solar_system_initializers/` | 39 | 49,425 | fixed, random, and custom-pool names attached during galaxy initialization |

Notable measured pools in `common/random_names/base/00_random_names.txt` are:

| Pool | Entries |
|---|---:|
| `star_names` | 1,748 (not necessarily unique) |
| `black_hole_names` | 67 |
| `asteroid_prefix` | 138 |
| `asteroid_postfix` | 129 |
| `nebula_names` | 55 |
| `cosmic_storm_name_prefixes` | 11 |
| species-modification prefixes/postfixes | 7 / 5 |

The weighted grammar files contain 161 empire part lists and 237 empire formats, 24 war part lists and 43 war formats, eight federation formats, and 19 pop-faction formats in this snapshot.

### 2.2 Evidence grades used in this report

- **Documented:** stated in a vanilla README, initializer documentation, or generated `script_documentation` log.
- **Directly observed:** present in the copied vanilla definitions.
- **Strong inference:** the data shape and in-game-facing field names make the behavior clear, but the closed engine implementation is unavailable.
- **Unknown:** cannot be established without engine source, controlled runtime experiments, or representative saves.

This distinction matters. ClauseScript describes inputs and policies; it is not the implementation of Stellaris's name manager.

### 2.3 Snapshot limitation

The copied `Paradox/vanilla` tree contains `common`, `map`, and `prescripted_countries`, but **not the game's `localisation` tree**. The report can prove which localization keys are referenced and how scoped localization is exposed, but it cannot audit every final English string or every translated equivalent from this snapshot alone.

---

## 3. System architecture

The runtime pipeline is best modeled as:

```text
definition discovery
  common/name_lists/*.txt
  common/species_names/*.txt
  common/random_names/**/*.txt
  initializers / presets / event script
            |
            v
catalog construction
  pools + weighted parts + weighted formats + triggers + fallbacks
            |
            v
contextual generation
  seeded RNG + object type + owner/species/country scope + used-name registry
            |
            v
abstract name value
  literal token, localization key, sequential template, or compound loc template
            |
            v
localization rendering
  language catalog + scoped commands + grammatical formatter
            |
            v
displayed and persisted entity name
```

There are three separate concerns that a reimplementation must not collapse:

- **Selection identity:** which pool item or format was chosen.
- **Semantic template:** the chosen token plus parameters, scopes, noun/adjective metadata, and sequence number.
- **Rendered text:** the language-specific string shown to a player at a particular time.

The local ClauseThing textbook notes that Stellaris 3.4+ saves often store names as abstract localization templates. That is consistent with the large number of `GetNamePersistent` properties in `script_documentation/localizations.log`: the engine distinguishes ordinary scoped lookup from a persistent name representation.

---

## 4. ClauseScript representation

### 4.1 The data language

Name data uses ordinary ClauseScript/Clausewitz syntax:

```clausewitz
KEY = {
    scalar = value
    list = { ITEM_A ITEM_B "ITEM WITH SPECIAL HANDLING" }
    repeated_key = first
    repeated_key = second
}
```

Relevant lexical properties are:

- `#` begins a comment.
- Whitespace separates unquoted tokens.
- Quotes preserve a scalar containing characters or syntax that should be treated as one token.
- A block may behave as a map, a list, or a mixed container depending on the field consuming it.
- Repeated keys are legal and often meaningful. Jomini explicitly models duplication, aliases, collection, and last-value selection.
- File order and duplicate order must be preserved until the consuming schema applies its policy.

The format is deliberately permissive and consumer-defined. Jomini's README warns that Clausewitz is closed source, lacks a complete public specification, and allows individual game objects to define specialized syntax. A generic parser can preserve structure; a Stellaris-aware semantic layer must interpret it.

### 4.2 Tokens are often localization identities

Examples such as `MAM1_SHIP_TTanak`, `SPEC_Alari`, `NAME_Earth`, and `MAM1_STARFLEET` are not prose embedded directly in the definition. They are identities intended to resolve through localization or name-template logic. Underscore-rich celestial tokens may also rely on localization/fallback formatting. A consumer must preserve the original token; eagerly replacing underscores or stripping prefixes destroys information.

The conventional localization package is a parallel tree under `localisation/<language>/` containing language-specific YAML files. A mod that adds name keys must ship those keys for every language it supports. Because that tree is absent from the copied snapshot, exact localization-file encoding and all final strings are outside this corpus audit.

### 4.3 Names are a special engine value, not a plain string everywhere

Generated documentation exposes:

- `set_name` on countries, planets, ships, fleets, systems, leaders, armies, pop factions, wars, federations, sectors, first-contact sites, and other scopes.
- `GetName` and often `GetNamePersistent` on most named scopes.
- species singular, plural, adjective, insult, compliment, anatomy, and related localized forms.
- leader first name, second name, regnal name, titled name, and persistent variants.
- country capital planet/system `OrRandom` accessors used before empire creation is complete.

This interface is richer than `String`. A faithful IR should represent literal, key, compound-template, scoped-reference, and sequence forms explicitly.

---

## 5. The standard `common/name_lists` schema

`common/name_lists/README_NAME_LISTS.txt` is the strongest source for the normal entity-name algorithm.

### 5.1 Name-list metadata and selection

A top-level definition is keyed by a stable identifier such as `MAM1`, `HUMAN1`, `MACHINE3`, or `PRT1`:

```clausewitz
MAM1 = {
    category = "Mammalian"
    ship_names = { ... }
    fleet_names = { ... }
    army_names = { ... }
    planet_names = { ... }
    character_names = { ... }
}
```

Optional metadata controls discovery:

- `selectable = { ... }`: availability in empire creation.
- `randomized = no`: exclude from random country-name-list selection; default is eligible.
- `alias = "Human"`: alternate script identifier; may be repeated.
- `trigger = { ... }`: country-scoped eligibility for random selection.
- `category = "Humanoid"`: queried by the `name_list_category` trigger.
- `customize_random_override = HUM1`: empire-designer species/homeworld/home-system randomization uses the named species database instead.
- `should_name_home_system_planets = no`: suppress thematic naming of non-home planets in the starting system.

The pirate list demonstrates context gating: `PRT1` is not player-selectable or generally randomized, and its trigger requires `is_pirate = yes`. Machine and hive lists suppress ordinary home-system planet naming. Human lists use aliases and customization overrides.

### 5.2 How a country/species acquires a list

Observed attachment paths include:

- `species { name_list = "HUMAN1" }` in `prescripted_countries`.
- `name_list = "HUM1"` inside a species-name bundle.
- `create_country { name_list = <key> }`.
- `create_species { namelist = random | random_class | scope }`.
- special countries created in initializers with an explicit `name_list`.

The selected list becomes cultural naming context. In ordinary use, leaders follow their species' list; ships, fleets, armies, and colonies use the owning country's list. Special scripts can override the name or attach a different list.

---

## 6. Entity algorithms

### 6.1 Characters and leaders

Character names are the clearest documented combinator.

```clausewitz
character_names = {
    culture_a = {
        weight = 40
        full_names = { ... }
        first_names_male = { ... }
        first_names_female = { ... }
        second_names = { ... }
        regnal_first_names = { ... }
        regnal_second_names = { ... }
        use_full_regnal_name = yes
    }
}
```

Documented generation procedure:

1. Select a `character_names` culture by weight (`1` by default). `HUMAN1` contains many weighted human cultural groups; most alien lists contain one `default` group.
2. If generating a ruler/heir under a government that uses regnal names, try the corresponding regnal pools; fall back to ordinary pools if no valid regnal result exists. `create_leader` can also request `use_regnal_name = yes`.
3. Select gender-specific pools when populated. For male/female leaders, fall back to the nongendered pool when the corresponding gender pool is empty. For indeterminate gender, select from available gendered pools (randomly between male and female when both exist), otherwise use nongendered values.
4. A result is either one entry from `full_names*`, or a concatenation of one `first_names*` entry and one `second_names*` entry.
5. If both a full-name route and a first+second route are valid, each route has a documented 50% chance.
6. `use_full_regnal_name*` controls whether localizations that normally abbreviate a two-part regnal name expose both parts.

This is combinatorial but not generative linguistics. If a culture has 100 valid first names and 100 second names, it exposes up to 10,000 pairings, all assembled from authored components.

Machine lists often use `full_names` only (for example `HelperbotA9`) and separate `regnal_full_names` such as `Caretaker` or `Coordinator`. Pirate leaders use authored sobriquets in `full_names`.

### 6.2 Ships

`ship_names` is a map from ship-size/type key to a flat pool:

```clausewitz
ship_names = {
    generic = { ... }
    corvette = { ... }
    science = { ... }
    colonizer = { ... }
    military_station_small = { ... }
}
```

Documented rules:

- `generic` may serve any ship size.
- If both generic and size-specific pools contain entries, the engine chooses between the two pool classes with a 50% chance, then selects an entry from the selected pool.
- Empty blocks are legal and mean no candidates for that specialization.

The creative content is authored. Aquatic lists contain different thematic pools for corvettes, destroyers, cruisers, science ships, constructors, and so on. Some older lists put nearly everything in `generic`.

`create_ship` accepts `name = random` and documents two additional controls:

- `prefix = yes` by default: apply the owner country's ship prefix.
- `suffix = yes`: allow numerals and add the name to the owner country's name-list bookkeeping.

The exact duplicate/retry/numeral algorithm is engine-private. It should not be fabricated from the presence of these flags.

Global event ship designs may bypass normal naming: `common/global_ship_designs/000_documentation.txt` exposes `use_design_name = yes`, which instructs ships of that design to use the fixed design name instead of a generated ship name.

### 6.3 Ship classes/design names

`ship_class_names` has the same generic and size-specific shape as `ship_names`. The README states that absent class-name candidates fall back to `ship_names`. This is distinct from naming an individual vessel.

### 6.4 Fleets

```clausewitz
fleet_names = {
    random_names = { ... }
    sequential_name = MAM1_STARFLEET
}
```

The engine first tries a `random_names` entry not already used by that country's fleets. When no unused authored name is available, it uses `sequential_name` with a number. Thus a fleet list is a finite unique pool with a guaranteed sequence-template fallback.

Special lists may omit `sequential_name`; the behavior after exhaustion is not documented by the local README and should be treated as an engine fallback/validation case.

### 6.5 Armies

Army naming is keyed by army type and has a `generic` fallback:

```clausewitz
army_names = {
    generic = { sequential_name = MAM1_INVADERCOLUMN }
    defense_army = { sequential_name = MAM1_DEFENSIVECOLUMN }
    clone_army = { sequential_name = MAM1_MOLDEDTROOPS }
}
```

Each type block may contain `random_names` and/or `sequential_name`. The engine uses `generic` when no type-specific definition exists. As with fleets, it prefers an unused random entry where possible and otherwise uses the numbered sequential template.

### 6.6 Colonies and thematic planet names

```clausewitz
planet_names = {
    generic = { names = { ... } }
    pc_desert = { names = { ... } }
    pc_ocean = { names = { ... } }
}
```

Documented behavior mirrors ships:

- `generic` works for any planet class.
- A planet-class block such as `pc_ocean` supplies biome-specific names.
- If both generic and class-specific candidates exist, there is a 50% chance of using either pool class.

Human lists use this to make biome-aware colonies (`Atlantis`-like ocean names, arctic names, and so forth). Most alien lists rely mainly on a generic themed pool. `should_name_home_system_planets` controls whether a starting country's list is also used for other bodies in its home system.

There is also a separate `custom_planet_names` registry in the base random-name file. This snapshot defines `exiled_colony_names`; scripts can request such a named special-purpose pool rather than a country's cultural list.

### 6.7 Species names

Species naming uses a separate database, not `character_names`. A representative entry is:

```clausewitz
MAM = {
    kroll = {
        name = SPEC_Kroll
        plural = SPEC_Kroll_pl
        home_planet = SPEC_Kroll_planet
        home_system = SPEC_Kroll_system
        name_list = "MAM1"
    }
}
```

The outer key is a species class. The inner entry is a pre-authored bundle coupling:

- singular species name;
- plural form;
- usually a home-planet key;
- usually a home-system key;
- usually a compatible cultural name list.

The five files in this snapshot contain 555 singular/plural bundles, 509 home-planet/home-system pairs, and 523 name-list links. Missing optional fields in special bundles imply fallback or caller-supplied values.

`create_species` documents `name = random` and `namelist = random` or `random_class`. `rename_species` can use a named species-database entry, `random`, or a specific name-list key. The database therefore performs coherent bundle selection: it does not synthesize a species word from syllables.

Modified species use the small authored `species_modification_prefix` and `species_modification_postfix` pools (`Neo-`, `Ultra-`, `Superior`, etc.). Exact choice and placement are engine code, but the available vocabulary is explicit.

### 6.8 Stars, systems, black holes, nebulas, storms, and asteroids

The base random-name file contains finite global pools:

- 1,748 `star_names` entries, mixing real astronomical names and invented names;
- 67 `black_hole_names`;
- 55 `nebula_names`;
- 11 localization keys for cosmic-storm prefixes;
- 138 asteroid prefixes and 129 postfixes.

The asteroid field names strongly indicate an engine combinator that selects one prefix and one postfix. If independent, the raw Cartesian space is 17,802 pairings before duplicate entries and any restrictions. The exact separator, collision handling, and RNG are not declared in script, so this is a strong inference rather than a source-level proof.

`common/solar_system_initializers/example.txt` documents:

- an initializer's `name` is optional; blank/default means a random system/main-star name;
- a scripted planet's `name` is optional; blank/default means random;
- names are applied as part of the engine's `InitializeSystem` pass before individual and initializer `init_effect` blocks;
- an `init_effect` may replace the generated name with `set_name`.

Initializers provide three modes:

1. **Blank/random:** normal global engine naming.
2. **Fixed key:** `name = "NAME_Zron"`, `NAME_Sol`, etc.
3. **Named special pool:** `namelist = "treasure_star_names"` or `"voidworm_star_names"`, backed by `custom_star_names` in the base file.

Ordinary unnamed planets in unowned systems are commonly rendered as derivatives of the system name plus an ordinal, and moons as derivatives of their parent. The available script proves the default is engine-generated but does not expose the ordinal-format implementation. This behavior should be verified from a generated save and localization catalog before cloning it.

### 6.9 Prescripted systems and entities

Prescripted countries and special initializers use direct keys for narrative stability:

```clausewitz
planet_name = "NAME_Earth"
system_name = "NAME_Sol"
ruler = { name = "PRESCRIPTED_ruler_name_humans1" }
```

Events and initializers similarly create countries, fleets, ships, and armies with fixed localization keys or `random`. Fixed names are not exceptions to the naming system; they enter through the same abstract name-value interface but skip random selection.

### 6.10 Sectors, starbases, megastructures, and other derived names

No dedicated global pools for sectors or ordinary starbases appear in `common/random_names` or `common/name_lists`. The generated docs expose name getters/setters for these scopes. Their normal defaults are therefore likely derived from a system, capital, design, or localization template in engine code, while special objects are explicitly named in script. This is an evidence gap, not proof that no other installed-game file participates.

---

## 7. Weighted grammar systems

The `common/random_names` grammar files are genuinely algorithmic. They still operate on curated tokens.

### 7.1 Generic grammar model

Each domain defines weighted part tables:

```clausewitz
empire_name_parts_list = {
    key = "imperial_mil"
    parts = {
        Empire = 3
        Imperium = 2
        Hegemony = 1
    }
}
```

and weighted formats:

```clausewitz
empire_name_format = {
    random_weight = {
        factor = 0
        modifier = {
            add = 5
            has_government = "gov_star_empire"
            is_pirate = no
        }
    }
    format = "{[This.GetSpeciesAdj] {<imperial_mil>}}"
    adjective = "{[This.GetSpeciesAdj]}"
    prefix_format = "{[This.GetSpeciesAdj] {<imperial_mil>}}"
}
```

A high-level reconstruction is:

```text
candidates = all formats for the domain
for each format:
    weight = evaluate(random_weight, current scope)
chosen_format = weighted_choice(candidates where weight > 0)
for each <parts_key> in chosen_format:
    chosen_part = weighted_choice(parts[parts_key])
for each [Scope.GetProperty]:
    obtain scoped abstract/localized value
result = expand localization formatter and nested arguments
attach optional adjective/noun/prefix metadata
```

`factor = 0` plus a conditional `modifier { add = N ... }` is an eligibility idiom: the format has zero weight unless its trigger succeeds. Large additions such as `999` make a specialized war format dominate a generic fallback.

The exact parser for `{...}`, `<parts_key>`, and formatter identifiers such as `AofB` is hardcoded name-format/localization machinery. It must not be treated as ordinary ClauseScript block syntax inside the quoted scalar.

### 7.2 Empire names

Empire formats condition on governments, ethics, civics, gestalt status, piracy, primitive status, fallen-empire status, and other country properties. They can draw from:

- weighted political nouns (`Empire`, `Republic`, `Coalition`, etc.);
- weighted descriptors (`United`, `Galactic`, `Free`, etc.);
- live species adjective;
- capital planet or system name;
- grammatical formatters such as “A of B”.

Formats may also produce `adjective`, `noun`, and `prefix_format` metadata. This lets the same generated identity support grammar and ship-prefix presentation rather than storing only one flattened label.

The file explicitly warns that empire creation may run before planet, system, or government choices exist. It requires `GetCapitalPlanetNameOrRandom` and `GetCapitalSystemNameOrRandom`, whose special behavior supplies a random placeholder when the designer has not selected those fields. `has_government` also has special empire-designer behavior.

### 7.3 War names

War grammar selects parts based on war goal and participant context. Examples distinguish conquest, corporate hostile takeover, subjugation, ideology, humiliation, cleansing, machine uprising, independence, federation-versus-federation, and other cases.

Formats inject attacker/defender adjectives or federation names:

```clausewitz
format = "{war_vs_adjectives {[This.MainAttacker.GetAdj] [This.MainDefender.GetAdj] <conquest_war_names>}}"
```

The algorithm is context-sensitive templating, not a stored table of every possible pair of belligerents.

### 7.4 Federation names

Federation formats are gated by federation type. A trade league may choose a descriptor, an economic term, and an organization noun; a research cooperative uses science/knowledge vocabulary; martial and hegemonic federations have their own pools. Two- and three-part combinations create many outputs from small curated lists.

### 7.5 Pop-faction names

Faction formats are gated by faction type. They may be a single authored phrase or a combination such as owner species adjective + ideological noun + organization noun:

```clausewitz
format = "{[This.Owner.GetSpeciesAdj] {<supremacist_names_1> {<supremacist_names_2>}}}"
```

This produces thematic variation while retaining a semantic link to the owner.

### 7.6 Pre-communications names

Unknown contacts use their own part lists and formats. The default combines a designation series (`Alpha`, `Beta`, etc.) with `Aliens`; xenophobes can use `Menace`; human presets have flag-gated vocabularies.

`cyclic = yes` appears on designation part lists. This directly indicates cycle-aware rather than independent-with-replacement use, although initial position, persistence scope, and wrap behavior remain engine details.

The separate shroud file contributes additional pre-communications part lists used for patron relations.

---

## 8. Localization and delivery

### 8.1 Scoped localization

`script_documentation/localizations.log` lists the hardcoded operations available inside localization text. Representative properties include:

- country: `GetName`, `GetAdj`, `GetSpeciesName`, `GetAllianceName`, `GetRulerName`, capital-name-or-random properties;
- planet/system: `GetName`, `GetStarName`, `GetClassName`;
- ship/fleet: ship, fleet, leader, star, design, size, and category names;
- leader: first, second, regnal, titled, and persistent names;
- species: singular, plural, adjective, anatomy, insult, and compliment forms;
- war/federation/faction/first-contact: domain name properties.

Square brackets begin a scoped localization expression, for example `[This.MainAttacker.GetAdj]`. The engine resolves promotions (`Owner`, `Species`, `Capital`, `Leader`, etc.) and properties at render/template-expansion time.

### 8.2 Persistent versus live lookup

Many scopes expose both `GetName` and `GetNamePersistent`. The logs do not define their binary representation, but the distinction and modern save behavior support a robust rule for tools:

- do not assume a saved `name` scalar is final prose;
- preserve template identity and parameters;
- load active base-game and mod localization catalogs to reproduce display text;
- retain a stable/persistent rendering path for historical entities and references whose live scope may disappear or change.

### 8.3 Packaging a mod

A name mod mirrors the game directory structure rather than editing vanilla files:

```text
my_name_mod/
  descriptor.mod
  common/
    name_lists/
      mymod_name_lists.txt
    species_names/
      mymod_species_names.txt          # if adding species bundles
    random_names/
      mymod_random_name_formats.txt    # if extending a recognized grammar domain
  localisation/
    english/
      mymod_names_l_english.yml
```

Prescripted empires or systems add their corresponding mirrored directories.

Engineering rules:

- use globally distinctive definition and localization prefixes;
- keep script identity separate from displayed text;
- ship every referenced localization key;
- include DLC/feature gating in `selectable`, `trigger`, or format weights where appropriate;
- test empire-designer randomization as well as runtime creation;
- avoid replacing an entire vanilla path unless total replacement is intended;
- treat cross-mod duplicate-definition/load-order behavior as domain-specific and test it in the target patch.

The local main modding summary explicitly instructs authors to create a mod folder and never modify base-game files. Exact launcher descriptor fields and load-order resolution are outside the copied naming corpus.

### 8.4 Distribution and synchronization

Content definitions and localization catalogs are package-time assets. At runtime the authoritative game chooses name identities/templates and associates them with objects. Multiplayer clients must agree on gameplay content and generated state, while each client can render through its language catalog. The available files do not expose network serialization or PRNG seeding, so exact synchronization mechanics are unknown.

For external save tooling, the practical distribution problem is explicit: name resolution may require the user's exact game version and active mods, not merely the save file.

---

## 9. What is and is not algorithmic

| Domain | Authored data | Algorithmic work | Engine-private pieces |
|---|---|---|---|
| Character | full/first/second/regnal pools, culture weights | weighted culture; gender fallback; 50/50 full vs combined; concatenate | RNG/seed, collision policy |
| Ship | generic and size/type pools | 50/50 generic vs specific; prefix/suffix; possible numerals | duplicate tracking and retry details |
| Ship class | generic and size pools | same selection; fallback to ship names | RNG details |
| Fleet | random pool + sequential template | unused random first, then numbered sequence | bookkeeping and formatting details |
| Army | type pools + generic fallback + sequence | type lookup, unused random, numbered fallback | bookkeeping details |
| Colony | generic and planet-class pools | 50/50 generic vs class-specific | collision handling |
| Species | coherent pre-authored bundles | class/context selection | random-entry selection details |
| Modified species | small prefix/postfix pools | attach a selected modifier | placement/fallback rules |
| Star/black hole/nebula | finite global pools | random selection | uniqueness and exhaustion |
| Asteroid | prefix and postfix pools | likely pairwise composition | separator, uniqueness, retries |
| Ordinary system bodies | global/system context | default random/ordinal derivation | exact format algorithm |
| Empire | weighted parts + 237 contextual formats | evaluate scope weights, choose format/parts, expand grammar | formatter implementation and PRNG |
| War | war-goal parts + 43 formats | contextual weighted template | formatter implementation and PRNG |
| Federation | themed parts + type formats | type-gated composition | formatter implementation and PRNG |
| Pop faction | ideological parts/formats | faction-gated composition | formatter implementation and PRNG |
| First contact | weighted/cyclic designation lists | context selection and cycling | cycle state/wrap details |
| Prescripted/special | fixed localization keys | none unless script says `random` | localization renderer |

---

## 10. Minimal authoring examples

### 10.1 A cultural name list

```clausewitz
MYMOD1 = {
    category = "My Culture"

    ship_names = {
        generic = { MYMOD_SHIP_Wayfarer MYMOD_SHIP_Resolute }
        science = { MYMOD_SHIP_Inquiry }
    }

    ship_class_names = {
        generic = { MYMOD_CLASS_Horizon }
    }

    fleet_names = {
        random_names = { MYMOD_FLEET_FirstWatch MYMOD_FLEET_FarWatch }
        sequential_name = MYMOD_FLEET_SEQ
    }

    army_names = {
        generic = { sequential_name = MYMOD_ARMY_SEQ }
        defense_army = { sequential_name = MYMOD_DEFENSE_SEQ }
    }

    planet_names = {
        generic = { names = { MYMOD_PLANET_Hearth MYMOD_PLANET_Haven } }
        pc_ocean = { names = { MYMOD_PLANET_Pelagos } }
    }

    character_names = {
        default = {
            first_names = { MYMOD_CHR_Aru MYMOD_CHR_Bela }
            second_names = { MYMOD_CHR_Venn MYMOD_CHR_Talar }
            regnal_full_names = { MYMOD_RULER_TheNavigator }
        }
    }
}
```

Every `MYMOD_*` token needs localization entries. The sequential entries must be templates compatible with the engine's numeric substitution convention; copying a current vanilla localization pattern from the actual installed localization tree is safer than guessing.

### 10.2 A species bundle

```clausewitz
MAM = {
    mymod_species = {
        name = MYMOD_SPEC_Name
        plural = MYMOD_SPEC_NamePlural
        home_planet = MYMOD_SPEC_Home
        home_system = MYMOD_SPEC_System
        name_list = "MYMOD1"
    }
}
```

This shows the key design principle: random empire setup selects a coherent bundle, not five unrelated strings.

### 10.3 A contextual grammar extension

```clausewitz
federation_name_parts_list = {
    key = "mymod_archive_terms"
    parts = {
        Archive = 3
        Repository = 2
        Memory = 1
    }
}

federation_name_format = {
    random_weight = {
        factor = 0
        modifier = {
            add = 100
            has_federation_type = mymod_archive_federation
        }
    }
    format = "{Galactic {<mymod_archive_terms>}}"
}
```

This only works if the target grammar domain recognizes the definition types and all literal/formatter tokens resolve correctly through localization.

---

## 11. Requirements for a faithful ClauseThing/SimThing implementation

### 11.1 Architectural conflict to resolve

The current `ClauseThing_Spec.md` explicitly classifies localization keys, name lists, bracket localizations, and other presentation constructs as permanent exclusions because SimThing lacked a display layer. This study shows that names are not cosmetic leaf strings: they are structured identity metadata used by empire creation, species bundles, ship prefixes, grammatical adjectives/nouns, historical persistence, and save analysis.

Therefore supporting Stellaris-style naming is **not** a small parser amendment. It requires either:

1. revising the exclusion and adding a presentation/identity subsystem to ClauseThing/Studio; or
2. creating a sibling `ClauseNameSpace` compiler that shares ClauseScript parsing but emits Studio metadata rather than simulation runtime artifacts.

The second option preserves the existing rule that `simthing-sim` never sees presentation data. It is the cleaner fit.

### 11.2 Proposed intermediate representation

```text
NameCatalog
  cultural_lists: Map<NameListId, CulturalNameList>
  species_bundles: Map<SpeciesClass, Vec<SpeciesNameBundle>>
  global_pools: Map<PoolId, WeightedPool<NameAtom>>
  grammars: Map<NameDomain, NameGrammar>
  localization: LocalizationCatalog

NameAtom
  Literal(text)
  LocalizationKey(key)
  ScopedProperty(scope_path, property)
  Compound(formatter, arguments)
  Sequential(template_key, ordinal)

GeneratedName
  atom
  adjective: optional atom
  noun: optional atom
  prefix_format: optional atom
  provenance: definition/file/list/format identifiers
  persistence: live | snapshot
```

`NameGrammar` should preserve ordered format definitions, trigger/weight expressions, part-list weights, and `cyclic` state declarations. `CulturalNameList` should use typed sections rather than an unstructured map so fallbacks can be validated.

### 11.3 Determinism contract

A new implementation must define behavior Stellaris keeps private:

- explicit seed source and deterministic PRNG;
- stable iteration order independent of hash-map ordering;
- weighted-choice behavior for zero/negative/overflowing totals;
- collision and exhaustion policy per entity domain;
- used-name registry scope (country, species, galaxy, pool, or object type);
- cycle cursor scope and wrap behavior;
- sequence numbering start and reuse policy;
- snapshot versus live scoped-reference semantics.

Reproducing *Stellaris outputs bit for bit* is impossible without identifying its PRNG and seed/call order. Reproducing the **authored semantics deterministically** is feasible and should be the product goal.

### 11.4 Parser requirements

The existing Jomini-grounded parser is suitable if the naming layer retains:

- duplicate top-level definitions and source order;
- list/map/mixed-block distinction;
- original quoted/unquoted scalar identity;
- comments only for diagnostics, not semantics;
- numeric weights without converting identifiers into prose;
- embedded format strings as a second language requiring a dedicated parser;
- trigger scopes and `random_weight` expressions;
- file and span provenance for errors.

Do not lower localization keys to ordinary strings during Stage 4. Do not evaluate scoped name references while compiling definitions.

### 11.5 Recommended compilation stages

1. Parse ClauseScript losslessly enough to preserve order, duplicates, and scalars.
2. Discover name-definition domains by directory and top-level key type.
3. Build typed cultural lists, species bundles, global pools, parts lists, and formats.
4. Resolve references (`name_list`, aliases, `<parts_key>`, localization keys where catalogs are present).
5. Compile trigger/weight expressions against a bounded naming context.
6. Validate fallback completeness and report empty/unreachable pools.
7. Generate abstract names with deterministic RNG and explicit registry state.
8. Render separately through a language-specific localization catalog.
9. Persist the abstract form plus provenance; optionally cache rendered text.

### 11.6 Validation suite

At minimum, add golden tests for:

- male, female, and indeterminate character fallback;
- full-name versus first+second 50/50 branch selection;
- weighted multi-culture selection;
- regnal fallback and full-regnal flags;
- generic versus size/class-specific ship and planet pools;
- fleet/army random exhaustion into sequential numbering;
- ship-class fallback to ship names;
- species bundle coherence;
- asteroid prefix/postfix composition;
- format eligibility and weighted part expansion;
- scope injection for empire, war, federation, and faction templates;
- cyclic pre-contact designators;
- missing localization and missing-pool diagnostics;
- stable output across repeated runs with the same seed;
- save/load round trip of abstract compound names.

Runtime probes against Stellaris should record a seed/save, generate many entities, and compare empirical selection frequencies and exhaustion behavior to this model.

---

## 12. Unknowns and experiments needed

The following cannot be settled from the supplied definitions:

1. Exact PRNG family, seed derivation, and call order.
2. Whether every nominally equal pool entry has exactly equal probability when no weights are written.
3. Exact duplicate-name retry limits and behavior after every fallback is exhausted.
4. Galaxy-wide versus owner-local uniqueness for celestial, ship, and colony names.
5. Exact asteroid join/separator rules and whether every prefix can pair with every postfix.
6. Default ordinal formats for unnamed planets, moons, binary stars, and stations.
7. `cyclic = yes` cursor initialization, shuffle behavior, persistence, and wrap.
8. Exact semantics of `GetName` versus `GetNamePersistent` in all scopes.
9. Cross-file and cross-mod merge/override policy for each name-definition class.
10. Localization fallback behavior for missing keys, quoting, underscore conversion, and non-English grammar.
11. Multiplayer serialization of generated compound names.

Recommended evidence collection:

- copy the installed `localisation` tree and one active mod's localization tree;
- generate controlled galaxies and inspect decompressed saves;
- create small test mods with two-entry weighted pools and deliberate duplicates;
- test pool exhaustion and save/reload;
- compare two clients using different display languages;
- inspect names before and after renaming a referenced capital/species to distinguish live from persistent template parameters.

---

## 13. Source index

Primary local sources used:

- `Paradox/vanilla/common/name_lists/README_NAME_LISTS.txt` — authoritative cultural-name-list schema and fallbacks.
- `Paradox/vanilla/common/name_lists/MAM1.txt`, `HUMAN1.txt`, `AQU1.txt`, `MACHINE3.txt`, `PRT1.txt` — representative alien, multi-culture human, ship-type/class, machine/regnal, and gated special lists.
- `Paradox/vanilla/common/species_names/species_00.txt`, `machine_names.txt`, and companions — coherent species bundles.
- `Paradox/vanilla/common/random_names/base/00_random_names.txt` — celestial pools, species-modification pieces, and custom star/planet pools.
- `Paradox/vanilla/common/random_names/00_empire_names.txt` — contextual empire grammars.
- `Paradox/vanilla/common/random_names/00_war_names.txt` — war-goal/participant grammars.
- `Paradox/vanilla/common/random_names/00_federation_names.txt` — federation-type grammars.
- `Paradox/vanilla/common/random_names/00_pop_faction_names.txt` — faction-type grammars.
- `Paradox/vanilla/common/random_names/00_pre_communications_names.txt` and `00_shroud_pre_communications_names.txt` — contact designators and cyclic lists.
- `Paradox/vanilla/common/solar_system_initializers/example.txt` — documented initialization order and default random naming.
- `Paradox/vanilla/common/solar_system_initializers/grand_archive_initializers.txt` and other initializers — named custom-pool and fixed-name usage.
- `Paradox/vanilla/prescripted_countries/00_top_countries.txt` — fixed empire/species/planet/system/ruler keys and cultural-list attachment.
- `Paradox/vanilla/common/global_ship_designs/000_documentation.txt` — design-name override.
- `Paradox/script_documentation/effects.log` — create/set/rename effects and random-name parameters.
- `Paradox/script_documentation/triggers.log` — `name_list_category` and contextual triggers.
- `Paradox/script_documentation/localizations.log` — scoped and persistent localization API.
- `jomini/README.md` — Clausewitz parsing, duplicate keys, mixed structures, encodings, and closed-format limitations.
- `ClauseThing_Spec.md` and `SimThing/docs/clausething/ClauseThing_Spec.md` — current ClauseThing boundary that excludes names/localization.
- `SimThing/docs/clausething/ClauseThing.md` — abstract localization templates in modern save-analysis workflows.

The local wiki summaries link to live Stellaris Wiki pages, but live web retrieval was blocked during this study. No unsupported detail was silently imported from those pages.

---

## 14. Final engineering judgment

Stellaris's naming system is a compact content runtime of its own. It combines authored catalogs, weighted grammar, scope-aware localization, uniqueness state, and persistent template values. Calling it either “just a list of strings” or “a procedural name generator” is incomplete:

- species, stars, and most entity names are finite curated data;
- characters, asteroids, sequences, and contextual political names use deterministic kinds of composition;
- empires, wars, federations, and factions are small weighted grammars over live game scope;
- localization is part of the data model, not a final cosmetic pass;
- the closed engine owns important operational semantics that the scripts do not reveal.

For ClauseThing, the correct design is to compile these files into a typed `NameCatalog` and abstract `GeneratedName` representation, then keep rendering and simulation separate. The simulation core does not need names, but Studio, authoring, saves, history, and mod interoperability do. That separation preserves SimThing's runtime firewall while admitting the identity layer Stellaris content actually depends on.
