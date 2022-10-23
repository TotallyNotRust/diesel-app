﻿// <auto-generated />
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Infrastructure;
using Microsoft.EntityFrameworkCore.Storage.ValueConversion;
using Opgave_2._8;

#nullable disable

namespace Opgave_2._8.Migrations
{
    [DbContext(typeof(databaseContext))]
    partial class databaseContextModelSnapshot : ModelSnapshot
    {
        protected override void BuildModel(ModelBuilder modelBuilder)
        {
#pragma warning disable 612, 618
            modelBuilder.HasAnnotation("ProductVersion", "6.0.10");

            modelBuilder.Entity("Opgave_2._8.Group", b =>
                {
                    b.Property<long>("GroupId")
                        .ValueGeneratedOnAdd()
                        .HasColumnType("INTEGER");

                    b.Property<string>("Name")
                        .HasColumnType("TEXT(32)");

                    b.HasKey("GroupId");

                    b.ToTable("Groups");
                });

            modelBuilder.Entity("Opgave_2._8.GroupUser", b =>
                {
                    b.Property<long>("GroupId")
                        .HasColumnType("INT");

                    b.Property<long>("UserId")
                        .HasColumnType("INT");

                    b.HasKey("GroupId", "UserId");

                    b.ToTable("GroupUser", (string)null);
                });

            modelBuilder.Entity("Opgave_2._8.User", b =>
                {
                    b.Property<long>("UserId")
                        .ValueGeneratedOnAdd()
                        .HasColumnType("INTEGER");

                    b.Property<string>("Password")
                        .HasColumnType("CHAR(36)");

                    b.Property<string>("Username")
                        .HasColumnType("VARCHAR(32)");

                    b.HasKey("UserId");

                    b.ToTable("User", (string)null);
                });
#pragma warning restore 612, 618
        }
    }
}
