---
name: OpenCoven Design System
description: Opinionated design constraints for OpenCoven harness interfaces, cockpit UIs, and agent-built components
style: Dark
accent: Violet
grid: 4px
framework: Lit/React
---

# OpenCoven Design System

Opinionated UI constraints for building consistent, dense, command-first interfaces for the OpenCoven harness ecosystem (comux, OpenMeow, OpenClaw integrations).

## Colors

**Semantic Tokens:**

```
# Foundation (Grayscale)
surface-0:        #000000  (pure black, backgrounds)
surface-1:        #0d0d0d  (raised, very dark)
surface-2:        #1a1a1a  (cards, panels)
surface-3:        #262626  (borders, dividers)
surface-4:        #333333  (hover states)

text-primary:     #ffffff  (main text)
text-secondary:   #b3b3b3  (secondary, muted)
text-tertiary:    #808080  (very muted, hints)
text-inverse:     #000000  (on accent backgrounds)

border-primary:   #333333  (dividers, outlines)
border-secondary: #1f1f1f  (subtle dividers)
border-focus:     #7c3aed  (focused elements)

# Accent (Violet)
accent-primary:   #7c3aed  (interactive, highlights, focus)
accent-secondary: #6d28d9  (hover state)
accent-dark:      #5b21b6  (active/pressed)
accent-light:     #a78bfa  (light overlays, secondary accent)

# Semantic
success:          #10b981  (confirmations, positive)
warning:          #f59e0b  (cautions, warnings)
error:            #ef4444  (destructive, errors)
info:             #7c3aed  (same as accent-primary)
```

## Typography

**Font Stack:**
```
--font-primary:   -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", sans-serif
--font-mono:      "Courier New", monospace, "SF Mono"
```

**Scale (rem units):**
```
text-xs:          0.75rem  (10px, labels, tags)
text-sm:          0.875rem (14px, secondary text)
text-base:        1rem     (16px, body text)
text-lg:          1.125rem (18px, section headings)
text-xl:          1.25rem  (20px, modal titles)
text-2xl:         1.5rem   (24px, page titles)
text-mono:        0.875rem (monospace for code, logs, terminal output)
```

**Weights:**
```
font-normal:      400
font-medium:      500  (labels, highlights)
font-semibold:    600  (headings, emphasis)
font-bold:        700  (page titles only)
```

**Text Styles:**
```
heading-1:        text-2xl, font-bold, line-height: 1.3
heading-2:        text-xl, font-semibold, line-height: 1.4
heading-3:        text-lg, font-semibold, line-height: 1.4
body:             text-base, font-normal, line-height: 1.6
caption:          text-xs, font-normal, line-height: 1.5
code:             text-mono, font-normal, background: surface-2, padding: 0.25rem 0.5rem
```

## Spacing

**Grid:** 4px base unit

**Scale:**
```
0:    0px
0.5:  2px   (micro-spacing)
1:    4px   (base)
1.5:  6px   (compact)
2:    8px   (standard, default margins)
3:    12px  (comfortable)
4:    16px  (section spacing)
6:    24px  (major breaks)
8:    32px  (page-level spacing)
12:   48px  (full-page gutters)
```

**Layout Gutters:**
```
--gutter-page:    1.5rem  (24px, outer margins)
--gutter-section: 1rem    (16px, section breaks)
--gutter-compact: 0.5rem  (8px, dense cockpit mode)
```

## Borders

**Radius Scale:**
```
radius-none:      0px
radius-sm:        2px   (subtle corners, tight components)
radius-md:        4px   (standard, cards, buttons)
radius-lg:        6px   (modals, popovers)
radius-xl:        8px   (large surfaces)
radius-full:      9999px (badges, avatars)
```

**Width Scale:**
```
border-1:         1px   (standard)
border-2:         2px   (focus rings, emphasis)
border-3:         3px   (destructive confirmation)
```

**Focus Rings:**
```
focus-ring:       2px solid accent-primary, offset 2px
focus-ring-error: 2px solid error, offset 2px
```

## Layout

**Viewport Breakpoints:**
```
mobile:   0px     (phones)
tablet:   768px   (iPad, small tablets)
desktop:  1024px  (laptop)
wide:     1440px  (ultrawide)
```

**Container Patterns:**
```
page-max-width:   1400px
sidebar-width:    280px  (or 320px in detail view)
modal-width:      500px  (standard), 600px (wide), 400px (compact)
drawer-width:     400px

# Cockpit-style dense layout
dense-columns:    8-12 columns, 4px gaps (terminal-like grid)
```

**Component Layout:**
```
button-height:    32px    (standard), 24px (compact), 40px (large)
input-height:     32px
card-padding:     16px    (standard), 12px (compact)
table-row-height: 40px    (comfortable), 32px (dense)
```

## Components

**Button:**
```
Sizes:
  - small:   height 24px, padding 4px 12px, text-xs, rounded-sm
  - default: height 32px, padding 8px 16px, text-sm, rounded-md
  - large:   height 40px, padding 12px 20px, text-base, rounded-md

Variants:
  - primary:   background accent-primary, text white, border none
  - secondary: background surface-2, text text-primary, border 1px border-primary
  - outline:   background transparent, border 2px accent-primary, text accent-primary
  - ghost:     background transparent, border none, text text-primary (hover: background surface-2)
  - danger:    background error, text white, border none
  
States:
  - default:   as specified
  - hover:     darken 5%, cursor pointer
  - active:    darken 10%, border-focus 2px
  - disabled:  opacity 50%, cursor not-allowed
  - loading:   spinner, opacity 70%, cursor wait
```

**Input / TextArea:**
```
Sizing:
  - height 32px, padding 8px 12px
  - border 1px border-primary, radius-md
  - background surface-2, text text-primary

States:
  - focus:     border-2 accent-primary, outline none, shadow: inset 0 0 0 3px accent-light (opacity 20%)
  - hover:     border-primary 1px (darken)
  - error:     border-2 error
  - disabled:  background surface-1, text-tertiary, cursor not-allowed
  - readonly:  background surface-1, text-secondary

Placeholder: color text-tertiary
```

**Card / Panel:**
```
Background:     surface-2
Border:         1px border-primary, radius-md
Padding:        16px (standard), 12px (compact)
Shadow:         none (dark mode: no shadows)
Dividers:       1px border-secondary between sections
```

**Table:**
```
Header:
  - background surface-3
  - font semibold, text-xs, text-secondary
  - padding 8px 12px, align left
  
Row:
  - height 40px (comfortable), 32px (dense)
  - border-bottom 1px border-secondary
  - padding 8px 12px
  
Cells:
  - text-secondary by default
  - hover: background surface-1 (very subtle)
  
Alternating rows: no striping (maintain dense clarity)
```

**Modal / Dialog:**
```
Backdrop:       background black, opacity 50%
Container:      background surface-0, border 1px border-primary, radius-lg
Header:         padding 16px, heading-2, border-bottom 1px border-secondary
Body:           padding 16px
Footer:         padding 12px 16px, border-top 1px border-secondary, text-right buttons

Close button:   top-right corner, 24px square, icon ghost
```

**Badge / Tag:**
```
Padding:        2px 8px
Radius:         radius-full
Font:           text-xs, font-medium
Colors:         background surface-3, text text-primary (default)
                background accent-light, text text-inverse (accent)
                background error, text white (danger)
```

**Sidebar / Navigation:**
```
Width:         280px (standard), 320px (expanded)
Background:    surface-1
Item height:   40px
Item padding:  8px 12px
Item radius:   radius-sm
Item hover:    background surface-2
Item active:   background accent-secondary, text accent-light, left-border 3px accent-primary
Dividers:      1px border-secondary
Section title: text-xs, font-semibold, text-tertiary, padding-top 12px, padding-left 12px
```

**Session / Tab-like Pattern:**
```
Tab container:  background surface-1, border-bottom 1px border-secondary
Tab:            padding 8px 16px, height 36px, border-bottom 2px transparent
Tab hover:      background surface-2
Tab active:     border-bottom-color accent-primary, text accent-light
Session pill:   background surface-2, padding 4px 12px, radius-sm, text-xs, border 1px border-secondary
Session active: background accent-secondary, text accent-light
```

## Interactive States

**Hover:**
```
Buttons:         opacity +10%, color darken 5%
Cards:           background lighten 1%, border border-secondary
Links:           text accent-light, underline
```

**Focus:**
```
All interactive: 2px solid accent-primary outline, offset 2px, radius-sm
Text inputs:     inset shadow 0 0 0 3px accent-light (opacity 20%)
Keyboard:        always visible (do not :focus { outline: none; })
```

**Disabled:**
```
Buttons:         opacity 50%, cursor not-allowed
Inputs:          opacity 50%, background surface-1, cursor not-allowed
Text:            text-tertiary
```

**Active / Pressed:**
```
Buttons:         background darker variant, scale 98%, shadow none
Toggles:         background accent-dark, border accent-primary
```

**Error State:**
```
Input border:    2px error
Input shadow:    inset 0 0 0 3px error (opacity 10%)
Text message:    text-xs, color error, margin-top 4px
```

## Animation

**Timing:**
```
quick:      150ms  (hover states, tooltips)
normal:     250ms  (modals, transitions)
slow:       400ms  (page-level, complex animations)
```

**Easing:**
```
ease-in-out: cubic-bezier(0.4, 0, 0.2, 1)  (standard)
ease-out:    cubic-bezier(0, 0, 0.2, 1)    (exits)
ease-in:     cubic-bezier(0.4, 0, 1, 1)    (enters)
```

**Transitions:**
```
color:         250ms ease-in-out
background:    250ms ease-in-out
border:        250ms ease-in-out
opacity:       150ms ease-in-out
transform:     200ms ease-out (scale, translate only)
```

**Reduced Motion:**
```
@media (prefers-reduced-motion: reduce):
  - all animations → 0ms (instant)
  - all transitions → 0ms (instant)
  - no transforms → instant repositioning
```

## MUST (Absolute Requirements)

- **DO use pure black (#000000) for backgrounds**, not #111111 or #1a1a1a elsewhere
- **DO use violet accent (#7c3aed) for all focus rings and primary CTAs**
- **DO NOT use random gradients or shadow effects** — flat design only
- **DO use the exact font stack** (system fonts, then Courier for monospace)
- **DO apply 4px grid spacing consistently** — all margins/padding in multiples of 4px
- **DO preserve focus rings at all times** — keyboard navigation must be visible
- **DO use the semantic color tokens exactly** — no arbitrary color values
- **DO handle destructive actions with error color (#ef4444) and confirmation gates**
- **DO maintain session/browser UI patterns** — tabs, history, forward/back paradigms where applicable
- **DO NOT strip disabled or readonly states** — always show UI feedback for unavailable actions

## SHOULD (Strong Recommendations)

- **Prefer flat design over bevels, shadows, or 3D effects**
- **Use dense layouts (cockpit-style) in admin/operator UIs** — tight grids, minimal spacing
- **Apply the monospace font to code, logs, terminal output, and API responses**
- **Use border-secondary (#1f1f1f) for subtle internal dividers, not border-primary**
- **Consider table row heights: 40px for comfortable reading, 32px for dense data**
- **Follow the text hierarchy:** heading-1 for page title, heading-2 for sections, heading-3 for subsections
- **Animate transitions smoothly** (250ms default) when state changes, but respect reduced-motion
- **Ensure modals and popovers have clear stacking** (backdrop, container, content layers)
- **Use badges and tags for status/metadata** — prefer inline pills over separate UI elements
- **Build sidebar navigation with clear active states** — always show current location

## NEVER (Explicit Prohibitions)

- **NEVER use light backgrounds or light mode styling** — pure dark theme only
- **NEVER apply random colors, gradients, or glow effects** — violates the Coven aesthetic
- **NEVER hide focus indicators** (no `outline: none` without replacement)
- **NEVER use shadows as primary design elements** — flat only
- **NEVER place text on complex backgrounds** — maintain contrast, flat backgrounds only
- **NEVER use Comic Sans, cursive, or decorative fonts** — system fonts only
- **NEVER break the 4px grid** — all spacing must be multiples of 4px
- **NEVER skip error states or disabled states** — all interactive elements must show feedback
- **NEVER hide destructive actions** — make them explicit with color + confirmation
- **NEVER use emojis or whimsical icons in system UI** — stay professional and minimal
- **NEVER apply animations to text transitions** — animate only position, opacity, and scale
- **NEVER use blue, green, or red as primary accents** — violet (#7c3aed) only

## Usage

### In Prompts
When building UI components, reference this skill:

```
You are building a Coven cockpit interface. Follow the OpenCoven Design System SKILL.md exactly.
Build a session manager card showing:
- Session ID (monospace)
- Status badge (success/warning/error)
- Delete button (destructive, confirmation required)
```

### In Code
Include the design tokens at the top of your CSS/JS:

```css
:root {
  --color-surface-0: #000000;
  --color-surface-1: #0d0d0d;
  --color-accent-primary: #7c3aed;
  --font-primary: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  --font-mono: "Courier New", monospace;
  --grid: 4px;
}
```

### In Design Tools
- **Figma:** Create component library matching these specs
- **Lit/React:** Build styled components using these tokens
- **Tailwind:** Extend config with Coven colors, spacing, and typography

## Examples

**Dense Dashboard (Cockpit Mode):**
- Sidebar (280px) + main content (flex)
- 4px grid spacing throughout
- Row height 32px
- Monospace for data cells
- Accent-colored status badges

**Modal with Destructive Action:**
- Header with title, close button
- Body with content
- Footer with Cancel (secondary) + Delete (danger, 2px red border)
- Confirmation: "Are you sure?" dialog required

**Session List:**
- Tab-like pills showing active session
- Each row: session ID (monospace), status (badge), actions (buttons)
- Hover: subtle background change, no animation
- Delete: error-colored, opens confirmation modal

---

**Reviewed by:** Sage (Research) + Coven Team  
**Last Updated:** 2026-05-10  
**Version:** 1.0.0  
**License:** MIT (OpenCoven)
