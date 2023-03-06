use crate::autogenfull::{
    Cell, BICOLOUR_TILES, BOTTOM_EDGES_BICOLOUR_ARRAY, BOTTOM_LEFT_CORNER_COLOUR_ARRAY,
    BOTTOM_RIGHT_CORNER_BICOLOUR_ARRAY, FILL_ORDER, LEFT_EDGES_COLOUR_ARRAY, MIDS_BICOLOUR_ARRAY,
    PREFILL_DEPTH, RIGHT_EDGES_BICOLOUR_ARRAY, TOP_EDGES_COLOUR_ARRAY, TOP_LEFT_CORNER_OFFSET,
    TOP_RIGHT_CORNER_COLOUR_ARRAY,
};
use crate::backtracker::{NUM_TILES, RECORD_DEPTH_SOLUTIONS, RECORD_DEPTH_STATS};
use crate::store;
use crate::Backtracker;
use crate::{
    east_colour, get_num_tiles, get_tile_id, is_tile_unplaced, iterate_possible_tiles, place_tile,
    record_count_at_depth, record_partial_solution_at_depth, record_solution, return_if_no_tiles,
    south_colour, try_next_cell, unplace_tile,
};

impl Backtracker<'_> {
    pub fn add_tile_0(&mut self, _depth: usize) {
        let tileoris_offset = TOP_LEFT_CORNER_OFFSET;
        let num_tiles = get_num_tiles!(tileoris_offset);

        if self.thread_params.is_mt_mode && PREFILL_DEPTH == 0 {
            let tiles_idx: usize = tileoris_offset + 2 + 4 * self.thread_params.tile_0_idx;
            self.add_tile_0_inner(tiles_idx);
        } else {
            for tiles_idx in iterate_possible_tiles!(self, tileoris_offset, num_tiles) {
                self.add_tile_0_inner(tiles_idx);
            }
        }
    }

    pub fn add_tile_top_right_corner(&mut self, depth: usize) {
        let tileoris_offset =
            TOP_RIGHT_CORNER_COLOUR_ARRAY[east_colour!(self, FILL_ORDER[depth])] as usize;

        let num_tiles = get_num_tiles!(tileoris_offset);
        return_if_no_tiles!(num_tiles);

        for tiles_idx in iterate_possible_tiles!(self, tileoris_offset, num_tiles) {
            let id = get_tile_id!(tiles_idx);

            if is_tile_unplaced!(self, id) {
                place_tile!(self, depth, id, tiles_idx);

                record_count_at_depth!(self, depth);
                record_partial_solution_at_depth!(self, depth);

                try_next_cell!(self, depth);

                unplace_tile!(self, id);
            }
        }
    }

    pub fn add_tile_bottom_left_corner(&mut self, depth: usize) {
        let tileoris_offset =
            BOTTOM_LEFT_CORNER_COLOUR_ARRAY[south_colour!(self, FILL_ORDER[depth])] as usize;

        let num_tiles = get_num_tiles!(tileoris_offset);
        return_if_no_tiles!(num_tiles);

        for tiles_idx in iterate_possible_tiles!(self, tileoris_offset, num_tiles) {
            let id = get_tile_id!(tiles_idx);

            if is_tile_unplaced!(self, id) {
                place_tile!(self, depth, id, tiles_idx);

                record_count_at_depth!(self, depth);
                record_partial_solution_at_depth!(self, depth);

                try_next_cell!(self, depth);

                unplace_tile!(self, id);
            }
        }
    }

    pub fn add_tile_top_edge(&mut self, depth: usize) {
        let tileoris_offset =
            TOP_EDGES_COLOUR_ARRAY[east_colour!(self, FILL_ORDER[depth])] as usize;

        let num_tiles = get_num_tiles!(tileoris_offset);

        for tiles_idx in iterate_possible_tiles!(self, tileoris_offset, num_tiles) {
            let id = get_tile_id!(tiles_idx);

            if is_tile_unplaced!(self, id) {
                place_tile!(self, depth, id, tiles_idx);

                record_count_at_depth!(self, depth);
                record_partial_solution_at_depth!(self, depth);

                try_next_cell!(self, depth);

                unplace_tile!(self, id);
            }
        }
    }

    pub fn add_tile_right_edge(&mut self, depth: usize) {
        let cell: &Cell = &FILL_ORDER[depth];

        let tileoris_offset = RIGHT_EDGES_BICOLOUR_ARRAY[south_colour!(self, cell)]
            [east_colour!(self, cell)] as usize;

        let num_tiles = get_num_tiles!(tileoris_offset);
        return_if_no_tiles!(num_tiles);

        for tiles_idx in iterate_possible_tiles!(self, tileoris_offset, num_tiles) {
            let id = get_tile_id!(tiles_idx);

            if is_tile_unplaced!(self, id) {
                place_tile!(self, depth, id, tiles_idx);

                record_count_at_depth!(self, depth);
                record_partial_solution_at_depth!(self, depth);

                try_next_cell!(self, depth);

                unplace_tile!(self, id);
            }
        }
    }

    pub fn add_tile_left_edge(&mut self, depth: usize) {
        let tileoris_offset =
            LEFT_EDGES_COLOUR_ARRAY[south_colour!(self, FILL_ORDER[depth])] as usize;

        let num_tiles = get_num_tiles!(tileoris_offset);

        for tiles_idx in iterate_possible_tiles!(self, tileoris_offset, num_tiles) {
            let id = get_tile_id!(tiles_idx);

            if is_tile_unplaced!(self, id) {
                place_tile!(self, depth, id, tiles_idx);

                record_count_at_depth!(self, depth);
                record_partial_solution_at_depth!(self, depth);

                try_next_cell!(self, depth);

                unplace_tile!(self, id);
            }
        }
    }

    pub fn add_tile_bottom_edge(&mut self, depth: usize) {
        let cell: &Cell = &FILL_ORDER[depth];

        let tileoris_offset = BOTTOM_EDGES_BICOLOUR_ARRAY[south_colour!(self, cell)]
            [east_colour!(self, cell)] as usize;

        let num_tiles = get_num_tiles!(tileoris_offset);
        return_if_no_tiles!(num_tiles);

        for tiles_idx in iterate_possible_tiles!(self, tileoris_offset, num_tiles) {
            let id = get_tile_id!(tiles_idx);

            if is_tile_unplaced!(self, id) {
                place_tile!(self, depth, id, tiles_idx);

                record_count_at_depth!(self, depth);
                record_partial_solution_at_depth!(self, depth);

                try_next_cell!(self, depth);

                unplace_tile!(self, id);
            }
        }
    }

    pub fn add_tile_mid(&mut self, depth: usize) {
        let cell: &Cell = &FILL_ORDER[depth];

        let tileoris_offset =
            MIDS_BICOLOUR_ARRAY[south_colour!(self, cell)][east_colour!(self, cell)] as usize;

        let num_tiles = get_num_tiles!(tileoris_offset);
        return_if_no_tiles!(num_tiles);

        for tiles_idx in iterate_possible_tiles!(self, tileoris_offset, num_tiles) {
            let id = get_tile_id!(tiles_idx);

            if is_tile_unplaced!(self, id) {
                place_tile!(self, depth, id, tiles_idx);

                record_count_at_depth!(self, depth);
                record_partial_solution_at_depth!(self, depth);

                try_next_cell!(self, depth);

                unplace_tile!(self, id);
            }
        }
    }

    pub fn add_tile_final(&mut self, depth: usize) {
        let cell: &Cell = &FILL_ORDER[depth];

        let tileoris_offset = BOTTOM_RIGHT_CORNER_BICOLOUR_ARRAY[south_colour!(self, cell)]
            [east_colour!(self, cell)] as usize;

        let num_tiles = get_num_tiles!(tileoris_offset);

        for tiles_idx in iterate_possible_tiles!(self, tileoris_offset, num_tiles) {
            let id = get_tile_id!(tiles_idx);

            if is_tile_unplaced!(self, id) {
                place_tile!(self, depth, id, tiles_idx);

                record_count_at_depth!(self, depth);
                record_partial_solution_at_depth!(self, depth);

                record_solution!(self);

                unplace_tile!(self, id);
            }
        }
    }
}
