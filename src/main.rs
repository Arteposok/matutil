#![windows_subsystem = "windows"]
use std::{collections::HashMap, default, fs};
use eframe::NativeOptions;
use egui::{FontData, FontDefinitions, FontFamily, Pos2, Rect, Vec2, ViewportBuilder};
use eval::Expr;

struct App{
    formula:String,
    x:String,
    y:String,
    z:String,
    eval_result:String,
    mean:String,
    mean_result:String,
    median:String,
    median_result:String,
}
impl App{
    fn eval_formula(&mut self){
        let value=Expr::new(self.formula.clone())
            .value("x", self.x.clone().parse().unwrap_or(1.))
            .value("y", self.y.clone().parse().unwrap_or(1.))
            .value("z", self.x.clone().parse().unwrap_or(1.))
            .exec().unwrap();
        let result=value.as_f64().unwrap() as f32;
        self.eval_result=format!("{}", result).to_string();

    }

    fn calculate_mean(&mut self){
        if self.mean.is_empty(){
            return
        }
        let values=self.mean.split(" ").into_iter().map(|x|{
            let result:f32=x.parse().unwrap_or(1.);
            result
        }).collect::<Vec<_>>();
        let sum:f32=values.iter().sum();
        let len=values.len() as f32;
        self.mean_result=format!("{}", sum/len).to_string();
    }

    fn calculate_median(&mut self){
        if self.median.is_empty(){
            return
        }
        let mut values=self.median.split(" ").into_iter().map(|x|{
            let result:f32=x.parse().unwrap();
            result
        }).collect::<Vec<_>>();
        values.sort_by(|x,y|{
            if x>y {
                std::cmp::Ordering::Greater
            }else if x<y {
                std::cmp::Ordering::Less
            }else{
                std::cmp::Ordering::Equal
            }
        });
        for x in values.clone(){
            print!("{}", x);
        }
        let len=values.len() as f32;
        if len%2.==0.{
            let ind=len/2.;
            let v1=values.get((ind+0.5) as usize).unwrap();
            let v2=values.get((ind-0.5) as usize).unwrap();
            self.median_result=format!("{}", (v1+v2)/2.).to_string();
        }else{
            let ind=len/2.;
            let v1=values.get((ind-0.5) as usize).unwrap();
            self.median_result=format!("{}", v1).to_string();
        }
    }
    fn configure_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert("my_font".to_owned(),
           std::sync::Arc::new(
               // .ttf and .otf supported
               FontData::from_static(include_bytes!(".././assets/font.ttf"))
           )
        );
        
        // Put my font first (highest priority):
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());
        
        // Put my font as last fallback for monospace:
        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());
        
        ctx.set_fonts(fonts);
    }
}

impl Default for App{
    fn default() -> Self {
        Self { 
            formula:String::default(),
            x:String::default(),
            y:String::default(),
            z:String::default(),
            mean:String::default(),
            median:String::default(),
            mean_result: String::default(),
            median_result: String::default(),
            eval_result: String::default()
         }
    }
}
impl eframe::App for App{
    fn update(&mut self, ctx_: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx_, |ctx|{
            Self::configure_fonts(ctx_);
            ctx.group(|ctx|{
                ctx.label("formula");
                ctx.text_edit_singleline(&mut self.formula);
                ctx.label("x");
                ctx.text_edit_singleline(&mut self.x).on_hover_text("x");
                ctx.label("y");
                ctx.text_edit_singleline(&mut self.y).on_hover_text("y");
                ctx.label("z");
                ctx.text_edit_singleline(&mut self.z).on_hover_text("z");
                if ctx.button("calculate").clicked(){
                    self.eval_formula();
                }
                ctx.label(self.eval_result.clone());
            });
            ctx.horizontal(|ctx|{
                ctx.group(|ctx|{
                    ctx.vertical(|ctx|{
                        ctx.label("mean");
                        ctx.text_edit_singleline(&mut self.mean);
                        if ctx.button("count").clicked(){
                            self.calculate_mean();
                        }
                        ctx.label(self.mean_result.clone());
                    });
                });
                ctx.group(|ctx|{
                    ctx.vertical(|ctx|{
                        ctx.label("median");
                        ctx.text_edit_singleline(&mut self.median);
                        if ctx.button("count").clicked(){
                            self.calculate_median();
                        }
                        ctx.label(self.median_result.clone());
                    });
                });
            });
        });
    }
}

fn main() {
    let mut options=NativeOptions::default();
    options.viewport=ViewportBuilder::default().with_inner_size(Vec2::new(650., 400.));
    let _result=eframe::run_native("matutil",
     options,
    Box::new(|_cc|Ok(Box::new(App::default()))) );
}
