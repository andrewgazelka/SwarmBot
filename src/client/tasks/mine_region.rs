// Copyright (c) 2021 Andrew Gazelka - All Rights Reserved.
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::{
    client::{
        pathfind::implementations::novehicle::TravelProblem,
        state::{global::GlobalState, local::LocalState},
        tasks::{
            compound::CompoundTask, lazy::LazyTask, navigate::NavigateProblem,
            safe_mine_coord::SafeMineRegion, stream::TaskStream, Task,
        },
    },
    protocol::InterfaceOut,
};

pub struct MineRegion;

impl TaskStream for MineRegion {
    fn poll(
        &mut self,
        _out: &mut impl InterfaceOut,
        local: &mut LocalState,
        global: &mut GlobalState,
    ) -> Option<Task> {
        let goal = global.mine.obtain_region()?;
        let start = local.physics.location();

        let mut compound = CompoundTask::default();
        let problem = TravelProblem::navigate_near_block(start.into(), goal, 0.0, false);
        let nav = NavigateProblem::from(problem);

        compound.add(nav).add(LazyTask::from(SafeMineRegion));

        Some(compound.into())
    }
}
