
use crate::imp::*;
use lib1::geometry::quad::*;
use lib1::geometry::two_dim::*;
use png;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::surface::Surface;
use sdl2::{self, keyboard::Keycode::*, *};
use sdl2::{pixels::Color, rect, rect::Point, render::Canvas, ttf::Font, video::Window};
use std::convert::TryInto;
use std::fs::File;
use std::ops::Deref;
use sdl2::audio::{AudioCallback,AudioSpecDesired};

pub fn func1(){
    let spec=AudioSpecDesired{};
    


let sdl_ctx=sdl2::init().unwrap();

let vid_ctx:AudioSubsystem= sdl_ctx.audio().unwrap();



}