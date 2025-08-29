## Title
Standardize file naming to GitHub conventions

## Description
Aligns project file structure with GitHub community standards. Addresses inconsistent naming that deviates from established conventions.

## Changes Made
- Renamed community health files to standard names
- Relocated issue templates to correct directory structure  
- Updated all internal file references
- Fixed minor content issues

## Files Affected
**Renamed:**
- `CONDUCT.md` → `CODE_OF_CONDUCT.md`
- `CONTRIBUTE.md` → `CONTRIBUTING.md`
- `.github/pullrequest.md` → `.github/pull_request_template.md`
- `.github/templates/` → `.github/ISSUE_TEMPLATE/`
- Issue template files renamed per GitHub standards

**Updated:**
- `scripts/README.md` - File path references
- Fixed typo in Code of Conduct enforcement section

## Impact
- Improves project discoverability through GitHub's community standards
- Ensures templates function correctly in GitHub interface
- Eliminates broken internal references
- Maintains consistency across project documentation

## Testing
- Verified all internal references resolve correctly
- Confirmed no remaining references to old file names
- Validated GitHub template functionality

## Type of Change
- Documentation/structure improvement
- No functional code changes
- No breaking changes