

## v0.1.0 (2026-11-04)

### Chore

 - <csr-id-50b5d0e18a05b2e7b1da475efbccecf2c883af71/> update to rust 1.97, cleanup lints

### New Features

 - <csr-id-bf64fb1788331c85f6de29cc334518d88e87f8ea/> Renderer sense:click => sense:all
 - <csr-id-14f0fb61f10300d120ab9e4acb5cd562716e6b2d/> update renderers for searching
 - <csr-id-6c02b3d7cd756a9d966dd1b073e820c3d6e7ae6e/> Add RootWalkPlacer
 - <csr-id-99fb5a060fc418ef4b30379e77f5ce20441c015c/> First pass implementation of search widget
   # Conflicts:
   #	libraries/graphing/src/egui/mod.rs
 - <csr-id-b8874518539c535fea96e41fc9436bff108e82c6/> Node gets some internal user state
 - <csr-id-c882b814d916653f0411365815f67a5ecae0c30b/> tweaks to FDP for perf and caching improvements
 - <csr-id-791cc469b363443f8488c2e94729ecf98968d07e/> making use of the new coordinate systems
 - <csr-id-a846ba39319ba03ab715456e784f39bcbbc3e70b/> improved rendering support with individualized responses
 - <csr-id-15b752f42f8491fc2e1664c4dc343e2896983baf/> magnetic edges
 - <csr-id-0fd86957822ddc8d12dbf5a99d29c2b4d35415e0/> upstream/downstream analysis
 - <csr-id-c8232ba054864c85ecf6607c37027cfe6692a538/> new walkers
 - <csr-id-3e306b44e9b9fb216487d17736192920026d0790/> graph gets merge_from
 - <csr-id-215a6e7926c819785158a671ad013c8fa75a35da/> add testing window for scroll interaction
 - <csr-id-79899a7f3ca3cb63807fff30f09dcdc7d3f43a99/> remove dragging code, fix iteration for filtered nodes
 - <csr-id-70ab70bf8bd0882d00a4dd30421de967e470dac9/> new example with multiple roots
 - <csr-id-5f11e8c60a6f2935e5b42e40ea39f9032f18bbec/> draw arrows on directed edges
 - <csr-id-f754289cd8b99e64a4a73c43ffe837089ba41374/> DFS and walking closure ecosystem
 - <csr-id-573407d1b5f54f6dd8e15fdd77758109f6136495/> easier graph filtering & inversion
 - <csr-id-333abd7124c3b6fbb1dc6d2cbb020e302108bbfc/> improved edge/node accessors
 - <csr-id-d67c036c0f05242446508659ad941a81c276787b/> graphing algorithms work - all-pairs shortest paths & root finding, new error type
 - <csr-id-51aa431a3f0e3f75ea16457103e65847010f493e/> finish rendering delegates for FDP
 - <csr-id-f64d398e98e43f34c0af09c7b050da9384f7431c/> beginnings of graph traversal
 - <csr-id-313b9f373391299a7bbf250062ee8ada0e84909e/> Improved OrderedHashMap apis
 - <csr-id-1f9d622e33b8ca8b97d9fe83846825aebe072059/> add profiling points
 - <csr-id-510dc2ca783c5b05967f75df1532033b0407f53d/> node dragging & parameters window
 - <csr-id-053eb15cd0ca6305bc998b8f687a4315dd798c94/> big rework for improved FDP perf
 - <csr-id-7d6363c16153ebe5b396ea80950722b2e973dc85/> tick controls and auto-repainting
 - <csr-id-24ed7c1464ebf2549c00a023dff25d60752245fa/> Open up simulation widget
 - <csr-id-5d0e37a914c3632b11a38b40f602ad10da1dc9cc/> FDP algorithm and FDP visualization

### Bug Fixes

 - <csr-id-0c18632e81afe11eac878420be2b38958544ef73/> zoom example function in disjoint
 - <csr-id-44ed6bc339f5bd55a01814323e629a49ffb0bdd1/> Search widget wasn't correctly matching cases
 - <csr-id-86bef1009345efdeb97bccae24e9c869aa09ca36/> CompositeRenderer now correctly delegates responses
 - <csr-id-a022ec3f433e13d0f42495a550a7480ecdde0078/> nope, that was a bad idea.  just update the edge counts on each tick once.
 - <csr-id-17ec769efac647f127d7a70225af3546362d66a4/> Push WorkingMem into the Node's memory to eliminate the 'num_edges' caching race condition
 - <csr-id-b2fc655f52159db795eee9090e2559576e026ff6/> ignoring forces test
 - <csr-id-abda5e0b2d47f86969ee2242c2c05346ebb656ce/> Fix force tests features
 - <csr-id-a6b13efa5b5146cc461094ef06e61bdc10d9cef1/> finally found the bug in Magnetic
 - <csr-id-e174f19dd89ad0d78a31d79bc991e94be1b603e0/> fix missing import
 - <csr-id-19b4d0d14e58be967b5d4120e3aba634abde0d4c/> adjust drag offset to align with widget
 - <csr-id-f0ebff0d369afd573b42406cd23fae598593d5e1/> fixing non-normal leaks in fdp math
 - <csr-id-9f05cb5db288407c4ab6da285a147b6b1ae0da1d/> pull autoplay out of controls block
 - <csr-id-9673164a8b55165ded3a7481ed6f959c678b1616/> missed a spot

### Other

 - <csr-id-450df9e778cdac79bd14da9f4b49064d3137ca6d/> adjusting node renderering
 - <csr-id-d47e51b9b5e9844b1817db388589094ead3c309e/> futzing with fdp parameters some more
 - <csr-id-5a2ef90e7bafeec40e35a61f68c34ad2935996b1/> units & graphing
 - <csr-id-626b51ace277f02658e47da7c4beeac66912ea40/> units & graphing

### Refactor

 - <csr-id-38d0d68cc8d1000d5cac9941d9b37ef970234455/> break out forces, add disjoint example

### New Features (BREAKING)

 - <csr-id-ac0d536ab5a136bcf33d49aeb96bc9fa9d153086/> another huge rework.  Identifiers everywhere now.  BFS & Pathing improvements.
 - <csr-id-170a4ac732fd9524798c20960fbe7106a76e65e9/> bump egui to 0.34

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 52 commits contributed to the release over the course of 324 calendar days.
 - 50 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update to rust 1.97, cleanup lints ([`50b5d0e`](https://github.com/spmadden/irox/commit/50b5d0e18a05b2e7b1da475efbccecf2c883af71))
    - Zoom example function in disjoint ([`0c18632`](https://github.com/spmadden/irox/commit/0c18632e81afe11eac878420be2b38958544ef73))
    - Search widget wasn't correctly matching cases ([`44ed6bc`](https://github.com/spmadden/irox/commit/44ed6bc339f5bd55a01814323e629a49ffb0bdd1))
    - Renderer sense:click => sense:all ([`bf64fb1`](https://github.com/spmadden/irox/commit/bf64fb1788331c85f6de29cc334518d88e87f8ea))
    - CompositeRenderer now correctly delegates responses ([`86bef10`](https://github.com/spmadden/irox/commit/86bef1009345efdeb97bccae24e9c869aa09ca36))
    - Update renderers for searching ([`14f0fb6`](https://github.com/spmadden/irox/commit/14f0fb61f10300d120ab9e4acb5cd562716e6b2d))
    - Add RootWalkPlacer ([`6c02b3d`](https://github.com/spmadden/irox/commit/6c02b3d7cd756a9d966dd1b073e820c3d6e7ae6e))
    - Nope, that was a bad idea.  just update the edge counts on each tick once. ([`a022ec3`](https://github.com/spmadden/irox/commit/a022ec3f433e13d0f42495a550a7480ecdde0078))
    - Revert "fix(graphing): Push WorkingMem into the Node's memory to eliminate the 'num_edges' caching race condition" ([`d2cefa2`](https://github.com/spmadden/irox/commit/d2cefa211fd3bec5faed59ca55039e4efa20685f))
    - Push WorkingMem into the Node's memory to eliminate the 'num_edges' caching race condition ([`17ec769`](https://github.com/spmadden/irox/commit/17ec769efac647f127d7a70225af3546362d66a4))
    - First pass implementation of search widget ([`99fb5a0`](https://github.com/spmadden/irox/commit/99fb5a060fc418ef4b30379e77f5ce20441c015c))
    - Node gets some internal user state ([`b887451`](https://github.com/spmadden/irox/commit/b8874518539c535fea96e41fc9436bff108e82c6))
    - Tweaks to FDP for perf and caching improvements ([`c882b81`](https://github.com/spmadden/irox/commit/c882b814d916653f0411365815f67a5ecae0c30b))
    - Making use of the new coordinate systems ([`791cc46`](https://github.com/spmadden/irox/commit/791cc469b363443f8488c2e94729ecf98968d07e))
    - Ignoring forces test ([`b2fc655`](https://github.com/spmadden/irox/commit/b2fc655f52159db795eee9090e2559576e026ff6))
    - Fix force tests features ([`abda5e0`](https://github.com/spmadden/irox/commit/abda5e0b2d47f86969ee2242c2c05346ebb656ce))
    - Improved rendering support with individualized responses ([`a846ba3`](https://github.com/spmadden/irox/commit/a846ba39319ba03ab715456e784f39bcbbc3e70b))
    - Finally found the bug in Magnetic ([`a6b13ef`](https://github.com/spmadden/irox/commit/a6b13efa5b5146cc461094ef06e61bdc10d9cef1))
    - Magnetic edges ([`15b752f`](https://github.com/spmadden/irox/commit/15b752f42f8491fc2e1664c4dc343e2896983baf))
    - Upstream/downstream analysis ([`0fd8695`](https://github.com/spmadden/irox/commit/0fd86957822ddc8d12dbf5a99d29c2b4d35415e0))
    - New walkers ([`c8232ba`](https://github.com/spmadden/irox/commit/c8232ba054864c85ecf6607c37027cfe6692a538))
    - Graph gets merge_from ([`3e306b4`](https://github.com/spmadden/irox/commit/3e306b44e9b9fb216487d17736192920026d0790))
    - Add testing window for scroll interaction ([`215a6e7`](https://github.com/spmadden/irox/commit/215a6e7926c819785158a671ad013c8fa75a35da))
    - Remove dragging code, fix iteration for filtered nodes ([`79899a7`](https://github.com/spmadden/irox/commit/79899a7f3ca3cb63807fff30f09dcdc7d3f43a99))
    - New example with multiple roots ([`70ab70b`](https://github.com/spmadden/irox/commit/70ab70bf8bd0882d00a4dd30421de967e470dac9))
    - Draw arrows on directed edges ([`5f11e8c`](https://github.com/spmadden/irox/commit/5f11e8c60a6f2935e5b42e40ea39f9032f18bbec))
    - DFS and walking closure ecosystem ([`f754289`](https://github.com/spmadden/irox/commit/f754289cd8b99e64a4a73c43ffe837089ba41374))
    - Easier graph filtering & inversion ([`573407d`](https://github.com/spmadden/irox/commit/573407d1b5f54f6dd8e15fdd77758109f6136495))
    - Improved edge/node accessors ([`333abd7`](https://github.com/spmadden/irox/commit/333abd7124c3b6fbb1dc6d2cbb020e302108bbfc))
    - Graphing algorithms work - all-pairs shortest paths & root finding, new error type ([`d67c036`](https://github.com/spmadden/irox/commit/d67c036c0f05242446508659ad941a81c276787b))
    - Another huge rework.  Identifiers everywhere now.  BFS & Pathing improvements. ([`ac0d536`](https://github.com/spmadden/irox/commit/ac0d536ab5a136bcf33d49aeb96bc9fa9d153086))
    - Finish rendering delegates for FDP ([`51aa431`](https://github.com/spmadden/irox/commit/51aa431a3f0e3f75ea16457103e65847010f493e))
    - Adjusting node renderering ([`450df9e`](https://github.com/spmadden/irox/commit/450df9e778cdac79bd14da9f4b49064d3137ca6d))
    - Beginnings of graph traversal ([`f64d398`](https://github.com/spmadden/irox/commit/f64d398e98e43f34c0af09c7b050da9384f7431c))
    - Bump egui to 0.34 ([`170a4ac`](https://github.com/spmadden/irox/commit/170a4ac732fd9524798c20960fbe7106a76e65e9))
    - Improved OrderedHashMap apis ([`313b9f3`](https://github.com/spmadden/irox/commit/313b9f373391299a7bbf250062ee8ada0e84909e))
    - Fix missing import ([`e174f19`](https://github.com/spmadden/irox/commit/e174f19dd89ad0d78a31d79bc991e94be1b603e0))
    - Add profiling points ([`1f9d622`](https://github.com/spmadden/irox/commit/1f9d622e33b8ca8b97d9fe83846825aebe072059))
    - Adjust drag offset to align with widget ([`19b4d0d`](https://github.com/spmadden/irox/commit/19b4d0d14e58be967b5d4120e3aba634abde0d4c))
    - Fixing non-normal leaks in fdp math ([`f0ebff0`](https://github.com/spmadden/irox/commit/f0ebff0d369afd573b42406cd23fae598593d5e1))
    - Break out forces, add disjoint example ([`38d0d68`](https://github.com/spmadden/irox/commit/38d0d68cc8d1000d5cac9941d9b37ef970234455))
    - Node dragging & parameters window ([`510dc2c`](https://github.com/spmadden/irox/commit/510dc2ca783c5b05967f75df1532033b0407f53d))
    - Futzing with fdp parameters some more ([`d47e51b`](https://github.com/spmadden/irox/commit/d47e51b9b5e9844b1817db388589094ead3c309e))
    - Big rework for improved FDP perf ([`053eb15`](https://github.com/spmadden/irox/commit/053eb15cd0ca6305bc998b8f687a4315dd798c94))
    - Release irox-tools v0.11.0 ([`b0589f9`](https://github.com/spmadden/irox/commit/b0589f9eae1dcd1486d9aa817406c1e9f9516d7f))
    - Pull autoplay out of controls block ([`9f05cb5`](https://github.com/spmadden/irox/commit/9f05cb5db288407c4ab6da285a147b6b1ae0da1d))
    - Missed a spot ([`9673164`](https://github.com/spmadden/irox/commit/9673164a8b55165ded3a7481ed6f959c678b1616))
    - Tick controls and auto-repainting ([`7d6363c`](https://github.com/spmadden/irox/commit/7d6363c16153ebe5b396ea80950722b2e973dc85))
    - Open up simulation widget ([`24ed7c1`](https://github.com/spmadden/irox/commit/24ed7c1464ebf2549c00a023dff25d60752245fa))
    - FDP algorithm and FDP visualization ([`5d0e37a`](https://github.com/spmadden/irox/commit/5d0e37a914c3632b11a38b40f602ad10da1dc9cc))
    - Units & graphing ([`5a2ef90`](https://github.com/spmadden/irox/commit/5a2ef90e7bafeec40e35a61f68c34ad2935996b1))
    - Units & graphing ([`626b51a`](https://github.com/spmadden/irox/commit/626b51ace277f02658e47da7c4beeac66912ea40))
</details>

