import gantt

# Change font default
gantt.define_font_attributes(
    fill='black', stroke='black', stroke_width=0, font_family="Verdana")


##########################$ MAKE DRAW ###############
p.make_svg_for_tasks(filename='test_full.svg', today=datetime.date(2014, 12, 31), start=datetime.date(2014, 8, 22), end=datetime.date(2015, 01, 14))
p.make_svg_for_tasks(filename='test_full2.svg',
                     today=datetime.date(2014, 12, 31))
p.make_svg_for_tasks(filename='test.svg', today=datetime.date(2014, 12, 31), start=datetime.date(2015, 01, 3), end=datetime.date(2015, 01, 06))
p1.make_svg_for_tasks(filename='test_p1.svg',
                      today=datetime.date(2014, 12, 31))
p2.make_svg_for_tasks(filename='test_p2.svg',
                      today=datetime.date(2014, 12, 31))
p.make_svg_for_resources(filename='test_resources.svg', today=datetime.date(
    2014, 12, 31), resources=[rANO, rJLS])
p.make_svg_for_tasks(filename='test_weekly.svg', today=datetime.date(
    2014, 12, 31), scale=gantt.DRAW_WITH_WEEKLY_SCALE)
##########################$ /MAKE DRAW ###############
